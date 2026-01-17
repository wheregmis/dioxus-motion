# Notes: Cleanup findings

## Baseline
- `cargo fmt --check`: clean
- `cargo test --features web`: passes
- `cargo clippy --features web -- -D warnings`: fails with a handful of lints

## Largest files (potential complexity hotspots)
- `src/pool.rs` (~1319 LOC)
- `src/animations/state_machine.rs` (~1012 LOC)
- `src/motion.rs` (~549 LOC)
- `src/animations/benchmarks.rs` (~525 LOC)
- `src/sequence.rs` (~397 LOC)

## Clippy issues to address first
- Unused imports in `src/manager.rs`
- Several `collapsible_if` suggestions (`src/animations/state_machine.rs`, `src/motion.rs`, `src/pool.rs`, `src/sequence.rs`)
- `derivable_impls` for `Default` on `AnimationState` (`src/animations/state_machine.rs`)
- `derivable_impls` for `Default` on `LoopMode` and `collapsible_if` in `AnimationConfig::execute_completion` (`src/animations/core.rs`)
- `unnecessary_option_map_or_else` in keyframe sorting (`src/keyframes.rs`)
- `unpredictable_function_pointer_comparisons` from deriving `PartialEq` on `Tween` (fn pointer field) (`src/animations/tween.rs`)

## Low-hanging complexity removed
- `Motion` had a `value_cache` field that was never populated (only ever set to `None`); removed the cache and related invalidation code paths.
