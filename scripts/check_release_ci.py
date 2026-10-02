"""Refuse publication unless the latest main push CI run passed for this commit."""

import json
import os
import subprocess


def require_successful_ci(ref, sha, runs):
    run = runs[0] if runs else {}
    if (
        ref != "refs/heads/main"
        or not sha
        or run.get("headSha") != sha
        or run.get("status") != "completed"
        or run.get("conclusion") != "success"
    ):
        raise SystemExit("Release requires successful push CI for this exact commit on main.")


if __name__ == "__main__":
    sha = os.environ["GITHUB_SHA"]
    runs = json.loads(subprocess.check_output([
        "gh", "run", "list", "--repo", os.environ["GITHUB_REPOSITORY"],
        "--workflow", "ci.yml", "--branch", "main", "--event", "push",
        "--commit", sha, "--limit", "1", "--json", "headSha,status,conclusion",
    ], text=True))
    require_successful_ci(os.environ["GITHUB_REF"], sha, runs)
