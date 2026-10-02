"""Run with python3 scripts/test_release_ci.py; no GitHub access required."""

from check_release_ci import require_successful_ci


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

print("Release CI gate checks passed")
