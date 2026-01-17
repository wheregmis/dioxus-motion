# Cleanup Report

## Summary
- Clippy is clean with `cargo clippy --features web -- -D warnings`.
- Removed a non-functional `Motion` value-cache that added complexity without benefit.
- Fixed a flaky perf assertion in `src/animations/benchmarks.rs` so CI isn't dependent on timing jitter.

## Follow-ups / Deeper simplifications
- Consider splitting `src/pool.rs` and `src/animations/state_machine.rs` into smaller modules with clearer responsibilities.
- Decide whether pooling/state-machine optimizations should be opt-in vs always-on.
