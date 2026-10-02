#!/usr/bin/env python3
"""Check browser delays with Chrome, matching ChromeDriver, and wasm-bindgen CLI."""
import functools
import json
import http.server
import os
from pathlib import Path
import shutil
import subprocess
import socket
import tempfile
import time
import urllib.request
import threading

root = Path(__file__).resolve().parents[1]
out = root / "target/browser-delay"
out.mkdir(parents=True, exist_ok=True)
wasm_bindgen = os.environ.get("WASM_BINDGEN", "wasm-bindgen")
chrome = os.environ.get("CHROME_BIN") or shutil.which("google-chrome") or shutil.which("chromium")
if not chrome and Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome").exists():
    chrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
if not chrome:
    raise SystemExit("Chrome is required; set CHROME_BIN to its executable.")
subprocess.run(["cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown",
                "--features", "web", "--example", "browser_delay_check"], cwd=root, check=True)
subprocess.run([wasm_bindgen, "--target", "web", "--out-dir", str(out),
                str(root / "target/wasm32-unknown-unknown/release/examples/browser_delay_check.wasm")], check=True)

class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass

handler = functools.partial(QuietHandler, directory=str(root))
with http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler) as server:
    threading.Thread(target=server.serve_forever, daemon=True).start()
    with tempfile.TemporaryDirectory(prefix="dioxus-motion-browser-") as profile:
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        driver_binary = os.environ.get("CHROMEDRIVER", "chromedriver")
        with (out / "chrome.log").open("w") as log:
            driver = subprocess.Popen([driver_binary, f"--port={port}"], stdout=log, stderr=log)
            session = None
            def request(method, path, data=None):
                body = json.dumps(data).encode() if data is not None else None
                req = urllib.request.Request(f"http://127.0.0.1:{port}{path}", data=body, method=method,
                    headers={"Content-Type": "application/json"})
                try:
                    with urllib.request.urlopen(req, timeout=30) as response:
                        return json.load(response)["value"]
                except urllib.error.HTTPError as error:
                    message = json.load(error)["value"]["message"]
                    raise RuntimeError(message) from None
            try:
                deadline = time.monotonic() + 10
                while True:
                    try:
                        request("GET", "/status")
                        break
                    except urllib.error.URLError:
                        if time.monotonic() >= deadline:
                            raise
                        time.sleep(0.1)
                session = request("POST", "/session", {"capabilities": {"alwaysMatch": {
                    "browserName": "chrome", "goog:chromeOptions": {"binary": chrome,
                    "args": ["--headless", "--no-first-run", "--use-mock-keychain", f"--user-data-dir={profile}"]}
                }}})["sessionId"]
                request("POST", f"/session/{session}/url", {"url": f"http://127.0.0.1:{server.server_port}/tests/browser_delay.html"})
                deadline = time.monotonic() + 15
                while True:
                    result = request("POST", f"/session/{session}/execute/sync",
                        {"script": "return document.querySelector('#result').textContent", "args": []})
                    if result != "RUNNING" or time.monotonic() >= deadline:
                        break
                    time.sleep(0.1)
                html = request("POST", f"/session/{session}/execute/sync",
                    {"script": "return document.documentElement.outerHTML", "args": []})
                (out / "result.html").write_text(html)
            finally:
                try:
                    if session:
                        request("DELETE", f"/session/{session}")
                finally:
                    driver.terminate()
                    try:
                        driver.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        driver.kill()
                        driver.wait()
    server.shutdown()
print(result)
if not result.startswith("PASS:"):
    raise SystemExit(1)
