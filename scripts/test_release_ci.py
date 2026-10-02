"""Run with python3 scripts/test_release_ci.py; no GitHub access required."""

import subprocess
import tempfile
from pathlib import Path

from check_release_ci import require_publication_checkout, require_successful_ci


passed = {"headSha": "tested-commit", "status": "completed", "conclusion": "success"}
require_successful_ci("refs/heads/main", "tested-commit", [passed])

# A successful older run must not hide a newer failure or incomplete rerun.
for ref, sha, runs in [
    ("refs/heads/feature", "tested-commit", [passed]),
    ("refs/tags/v0.4.0", "tested-commit", [passed]),
    ("refs/heads/main", "", [passed]),
    ("refs/heads/main", "untested-commit", [passed]),
    ("refs/heads/main", "tested-commit", []),
    ("refs/heads/main", "tested-commit", [{}]),
    ("refs/heads/main", "tested-commit", [dict(passed, status="in_progress"), passed]),
    *[("refs/heads/main", "tested-commit", [dict(passed, conclusion=c), passed])
      for c in ("failure", "cancelled", "skipped", "action_required", None)],
]:
    try:
        require_successful_ci(ref, sha, runs)
    except SystemExit:
        pass
    else:
        raise AssertionError(f"Accepted an unsafe release: {ref}, {sha}, {runs}")

require_publication_checkout("tested-commit", "tested-commit", "1")
for sha, head, commit_count in [
    ("", "", "1"),
    ("tested-commit", "pr-commit", "1"),
    ("tested-commit", "tested-commit", "2"),
]:
    try:
        require_publication_checkout(sha, head, commit_count)
    except SystemExit:
        pass
    else:
        raise AssertionError("Accepted a checkout that could publish another commit")

# release-plz 0.3.169 rewinds only when merge-base --is-ancestor succeeds.
# Keep the PR commit available via a tag to cover fetch-tags: true as well.
with tempfile.TemporaryDirectory() as tmp:
    origin = Path(tmp) / "origin"
    origin.mkdir()

    def git(*args, cwd=origin):
        return subprocess.check_output(["git", *args], cwd=cwd, text=True, stderr=subprocess.DEVNULL).strip()

    git("init", "--initial-branch=main")
    git("config", "user.name", "Release CI test")
    git("config", "user.email", "release-ci@example.invalid")
    git("commit", "--allow-empty", "-m", "base")
    git("checkout", "-b", "release-plz-test")
    git("commit", "--allow-empty", "-m", "prepared release")
    prepared = git("rev-parse", "HEAD")
    git("tag", "prepared")
    git("checkout", "main")
    git("merge", "--no-ff", "release-plz-test", "-m", "merge release")
    merged = git("rev-parse", "HEAD")
    git("merge-base", "--is-ancestor", prepared, merged)  # Full history would rewind.

    checkout = Path(tmp) / "checkout"
    git("clone", "--depth=1", "--no-single-branch", origin.as_uri(), str(checkout))
    assert git("rev-parse", "prepared", cwd=checkout) == prepared
    require_publication_checkout(
        merged, git("rev-parse", "HEAD", cwd=checkout),
        git("rev-list", "--count", "HEAD", cwd=checkout),
    )
    assert subprocess.run(
        ["git", "merge-base", "--is-ancestor", prepared, merged], cwd=checkout,
    ).returncode == 1  # Object exists, but is not an ancestor across the shallow boundary.

print("Release CI gate checks passed")
