# Dioxus Motion Benchmarks

This directory contains performance benchmarks for the dioxus-motion library.

## Running Benchmarks

To run the benchmarks, use:

```bash
cargo bench
```

## Benchmark Categories

### 1. Animation Updates (`animation_updates.rs`)
- Spring updates (web fixed-timestep, native RK4)
- Tween updates
- Transform interpolation (SIMD-optimized)
- Color interpolation

### 2. Store Operations (`store_ops.rs`)
- Store creation
- Animation dispatch (simple, keyframes, sequence)
- Signal updates and subscriptions

### 3. Keyframe & Sequence (`keyframes_sequence.rs`)
- Keyframe lookup and interpolation
- Sequence step transitions
- Complex animation chains

## Performance Goals (v0.4.0)

| Operation | Target Time | Notes |
|-----------|-------------|-------|
| Spring update | < 2μs | Per frame |
| Tween update | < 500ns | Per frame |
| Transform interpolation | < 100ns | SIMD-optimized |
| Keyframe lookup | < 200ns | Linear search |
| Sequence step | < 100ns | Index increment |

## Regression Testing

Before making performance-related changes:

1. Run benchmarks and save results: `cargo bench -- --save-baseline before`
2. Make your changes
3. Compare against baseline: `cargo bench -- --baseline before`

Look for regressions > 10% in hot paths.

## Adding New Benchmarks

When adding new benchmarks:

1. Use `criterion` for statistical rigor
2. Use `black_box` to prevent compiler optimizations
3. Use realistic input data
4. Add a baseline comparison if changing existing code

Example:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("my operation", |b| {
        b.iter(|| {
            // Your operation here
            black_box(expensive_operation());
        });
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
```

## Notes

- Benchmarks are currently optional and not run in CI
- For production performance, test on target platforms (web, native)
- Use browser DevTools for web-specific profiling
- Consider memory allocations, not just CPU time

