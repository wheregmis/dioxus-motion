# Contributing to Dioxus Motion

Thank you for your interest in contributing to Dioxus Motion! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/wheregmis/dioxus-motion.git
   cd dioxus-motion
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run clippy checks:
   ```bash
   cargo clippy --all-features -- -D warnings
   ```

## CI/CD Pipeline

We use GitHub Actions for continuous integration. The CI pipeline runs on every pull request to the main branch and includes:

### Comprehensive CI Checks
- **Compilation Check**: Checks the workspace with all features and on WASM
- **MSRV Check**: Checks published crates on Rust 1.89 for native and WASM
- **Clippy Check**: Enforces Rust coding standards and catches common issues
- **Test Suite**: Runs all unit and integration tests
- **Formatting Check**: Ensures code follows rustfmt standards
- **Security Audit**: Scans for known vulnerabilities
- **Multi-platform Testing**: Tests on Ubuntu, Windows, and macOS
- **Feature Matrix**: Tests all feature combinations (web, desktop, transitions)
- **Documentation Build**: Validates documentation generation

## Code Quality Standards

### Rust Standards
- All code must compile without warnings
- Clippy warnings are treated as errors
- Code must be formatted with `rustfmt`
- All tests must pass

### Commit Guidelines
- Use conventional commit messages
- Keep commits focused and atomic
- Include tests for new features
- Update documentation as needed

### Pull Request Process
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all CI checks pass
6. Submit a pull request

## Testing

### Running Tests Locally
```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features web
cargo test --features desktop
cargo test --features transitions

# Run tests with all features
cargo test --all-features
```

### Running Clippy
```bash
# Run clippy with all features
cargo clippy --all-features -- -D warnings

# Run clippy on workspace
cargo clippy --workspace --all-features -- -D warnings
```

### Performance and adversarial checks

Run manual timing measurements with optimizations enabled:

```bash
cargo test -p dioxus-motion --release --lib --locked test_motion_update_cpu_usage -- --ignored --nocapture
cargo test -p dioxus-motion --release --lib --locked test_keyframe_lookup_cpu_usage -- --ignored --nocapture
cargo test -p dioxus-motion --lib --locked fuzz_
```

The update benchmark keeps playback active and reports the median of seven samples,
with 100,000 updates per sample. `get_value` is measured separately because it clones
the value; combining it with updates hides map allocation costs. Compare changes on
the same host with the same harness and inputs. These timings exclude reactive
notifications, CSS formatting, DOM updates, and rendering.

On ARM64 macOS on 2026-10-02, reusing owned property maps reduced the two-property
`MotionStyle` spring case from 2,115 ns/update to 873 ns/update in a paired run
(about 59% less time). Both sides used the separated-read harness above, seven
samples, and 100,000 updates per sample. This is a host-specific baseline for that
case, not an end-to-end browser or many-component performance guarantee.

The shadow-and-filter case exercises complex numeric/color tokens separately.
On the same host and date, scaling those tokens in place reduced a paired run from
3,620 to 3,060 ns/update (about 15% less time), using the same sample settings.

The deterministic fuzz checks sample float bit patterns for spring coefficients,
values (including raw RGBA components), and frame deltas, plus Unicode code points
for CSS parsing and serialization.
They check finite state, preservation of the last valid value when a frame fails,
and parsing without UTF-8 slicing panics; they do not prove correctness for every
input or platform.

With `cargo-mutants` installed, audit a changed function, for example:

```bash
cargo mutants --cap-lints true --file src/animations/style.rs --re 'merge_style_properties|std::ops::Mul' -- --lib --locked
cargo mutants --cap-lints true --file src/motion.rs --re 'update_keyframes' -- --lib --locked
```

On 2026-10-02, the keyframe update audit caught all 33 generated mutations, with
zero survivors, timeouts, or unviable cases. Coverage includes held frames,
duplicate terminal offsets, zero duration, invalid easing, custom interpolation
failures, and agreement between binary lookup and an independent linear reference.
This result covers that function and those mutations; it is not a whole-library
correctness proof.

Inspect survivors and build failures individually. A timeout or unviable mutation
is not a caught mutation. Mutable selector callbacks required by Dioxus are private
and exposed through `ReadStore`; mutations affecting only those writers are not
reachable through the public motion API. Keep the public compile-fail checks that
prevent callers from writing invalid state, rather than adding an internal writer
test to raise the mutation score.

## Feature Development

### Adding New Features
1. Create a new branch for your feature
2. Implement the feature with appropriate tests
3. Update documentation
4. Ensure all CI checks pass
5. Submit a pull request

### Feature Flags
The project uses feature flags to control functionality:
- `web`: Web platform support (default)
- `desktop`: Desktop platform support
- `transitions`: Page transition support

## Documentation

### Code Documentation
- All public APIs must be documented
- Use Rust doc comments (`///`)
- Include examples in documentation
- Update README.md for user-facing changes

### Testing Documentation
- Write clear test descriptions
- Include edge case tests
- Test both success and failure scenarios

## Release Process

Release preparation uses `release-plz`; publication stays manual:

1. Merge changes to `main`. Release-plz opens or updates a release PR.
2. Review version bumps and both crate changelogs, and merge only after CI passes.
   If GitHub shows **Approve workflows to run** on the release PR, approve the
   pending workflows so the required CI checks run before merging.
   Require the CI jobs (including `Check MSRV (1.89)` and `Check release tooling`)
   in the repository's branch protection rules. Replace the retired
   `Check workspace members` requirement with the consolidated `Check` job.
3. Wait for the merged commit's **push** CI run to succeed, then dispatch
   **Release-plz** on `main`. Publication checks that exact commit's latest push CI
   run, verifies packaged code with all features, and uses `CARGO_REGISTRY_TOKEN`
   from the `cargo` GitHub environment.
   The publication job uses a depth-one checkout of `GITHUB_SHA` and checks that
   checkout before querying CI. This prevents pinned release-plz from selecting an
   earlier PR commit after a merge commit; release PR generation still fetches full
   history. Keep publication shallow when updating release-plz and verify its
   [commit-selection behavior](https://release-plz.dev/docs/usage/release#what-commit-is-released).
   A failed, pending, cancelled, missing, or unapproved run blocks publication.

The next main crate release is `0.4.0` because it changes public APIs. The transition
proc-macro source is unchanged from `0.1.2`; its version remains independent.
Keep unpublished notes in `[Unreleased]` until release-plz prepares them. The old
`0.3.6` heading described an unpublished release and has been folded back into
`[Unreleased]`. Avoid merging unrelated changes between preparing and publishing
an unpublished version: release-plz does not recalculate a version already ahead
of crates.io. Check breaking changes manually, particularly declarative and proc
macros that cargo-semver-checks cannot fully validate.

### Maintainer setup before merging these workflow changes

- Release PR creation uses the built-in `GITHUB_TOKEN` with **Contents: write**
  and **Pull requests: write**. Enable **Allow GitHub Actions to create and approve
  pull requests** in repository Actions settings. Bot-created PR workflows may
  require approval; see [GitHub's workflow triggering rules](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow#triggering-a-workflow-from-a-workflow).
- Store `CARGO_REGISTRY_TOKEN` in the `cargo` GitHub environment with permission
  to publish both `dioxus-motion` and `dioxus-motion-transitions-macro`. The manual
  publication job selects that environment and passes the token only to its
  release-plz step.
- Close the stale release PR #69 before merging, then let the next `main` push
  prepare a fresh release PR for `0.4.0` using the repaired changelogs.

Release-plz is pinned to `0.3.169` in the workflow's `RELEASE_PLZ_VERSION` variable.
Update that pin deliberately after reviewing its release notes. Validate local
release gate changes with `python3 scripts/test_release_ci.py` and workflow changes
with `actionlint`.

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in GitHub issues
- Check existing documentation

## License

By contributing to Dioxus Motion, you agree that your contributions will be licensed under the MIT License.
