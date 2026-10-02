//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance improvements
//! from closure pooling on web platforms and sleep optimization on desktop.

#[cfg(test)]
mod tests {
    #![allow(clippy::uninlined_format_args)]
    use instant::{Duration, Instant};

    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_keyframe_lookup_cpu_usage() {
        use crate::{KeyframeAnimation, Motion};
        use std::hint::black_box;

        const ITERATIONS: usize = 100_000;
        for count in [2, 32, 1024] {
            let mut animation = KeyframeAnimation::new(Duration::from_secs(10_000));
            for index in 0..count {
                animation = animation
                    .add_keyframe(index as f32, index as f32 / (count - 1) as f32, None)
                    .expect("finite keyframe offset");
            }
            let mut samples = Vec::with_capacity(7);
            for _ in 0..7 {
                let mut motion = Motion::new(0.0f32).expect("finite initial value");
                motion
                    .animate_keyframes(animation.clone())
                    .expect("valid keyframe setup");
                let start = Instant::now();
                for _ in 0..ITERATIONS {
                    black_box(
                        motion
                            .update(black_box(1.0 / 60.0))
                            .expect("representable animation frame"),
                    );
                    black_box(motion.get_value());
                }
                samples.push(start.elapsed());
                assert!(motion.is_running());
            }
            samples.sort_unstable();
            println!(
                "{count} keyframes: {:.2} ns/update (median of 7 samples)",
                samples[3].as_nanos() as f64 / ITERATIONS as f64,
            );
        }
    }

    /// Test desktop sleep optimization performance
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    #[ignore = "manual scheduling measurement; latency depends on host load"]
    async fn test_desktop_sleep_performance() {
        use crate::animations::platform::{MotionTime, TimeProvider};

        let test_durations = vec![
            Duration::from_micros(500), // Short - should yield
            Duration::from_millis(1),   // Threshold - should sleep
            Duration::from_millis(5),   // Medium - should sleep
        ];

        for duration in test_durations {
            let start = Instant::now();
            MotionTime::delay(duration).await;
            let elapsed = start.elapsed();

            // Validate performance characteristics
            if duration < Duration::from_millis(1) {
                // Very short durations should complete quickly
                assert!(
                    elapsed < Duration::from_millis(2),
                    "Duration {:?} took too long: {:?}",
                    duration,
                    elapsed
                );
            } else {
                // Longer durations should be reasonably accurate
                let tolerance = Duration::from_millis(3);
                assert!(
                    elapsed >= duration.saturating_sub(tolerance),
                    "Duration {:?} was too short: {:?}",
                    duration,
                    elapsed
                );
                assert!(
                    elapsed <= duration + tolerance,
                    "Duration {:?} was too long: {:?}",
                    duration,
                    elapsed
                );
            }
        }
    }

    /// Reports median update cost with animations active throughout each sample.
    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_motion_update_cpu_usage() {
        use crate::Motion;
        use crate::animations::core::{Animatable, AnimationMode};
        use crate::prelude::{AnimationConfig, LoopMode, Spring, Tween};
        use crate::prelude::{Color, Transform};
        use std::hint::black_box;

        const ITERATIONS: usize = 100_000;
        const DT: f32 = 1.0 / 60.0;

        let test_cases = [
            ("idle", None),
            (
                "running_tween",
                Some(AnimationConfig::new(AnimationMode::Tween(Tween::default()))),
            ),
            (
                "running_spring",
                Some(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
            ),
        ];

        fn measure<T: Animatable + Send + Copy>(
            value_type: &str,
            initial: T,
            target: T,
            test_cases: &[(&str, Option<AnimationConfig>)],
        ) {
            for (name, config) in test_cases {
                let active = config.is_some();
                let mut samples = Vec::with_capacity(7);
                for _ in 0..7 {
                    let mut motion = Motion::new(initial).expect("finite initial value");
                    if let Some(config) = config.clone() {
                        motion
                            .animate_to(target, config.with_loop(LoopMode::Infinite))
                            .expect("valid animation configuration");
                    }
                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        black_box(
                            motion
                                .update(black_box(DT))
                                .expect("representable animation frame"),
                        );
                        black_box(motion.get_value());
                    }
                    samples.push(start.elapsed());
                    assert_eq!(motion.is_running(), active);
                }
                samples.sort_unstable();
                println!(
                    "{value_type}/{name}: {:.2} ns/update (median of 7 samples, {ITERATIONS} updates/sample)",
                    samples[3].as_nanos() as f64 / ITERATIONS as f64,
                );
            }
        }
        measure("f32", 0.0f32, 100.0, &test_cases);
        measure(
            "Color",
            Color::new(1.0, 0.5, 0.25, 1.0),
            Color::new(0.0, 0.0, 0.0, 0.0),
            &test_cases,
        );
        measure(
            "Transform",
            Transform::identity(),
            Transform::new(100.0, 50.0, 2.0, 1.0),
            &test_cases,
        );
    }

    /// Integration test to verify the simplified motion loop remains deterministic
    #[test]
    fn test_motion_behavior_consistency() {
        use crate::Motion;
        use crate::animations::core::AnimationMode;
        use crate::prelude::{AnimationConfig, Tween};

        const DT: f32 = 1.0 / 60.0;
        const ANIMATION_STEPS: usize = 120; // 2 seconds at 60fps

        // Create two identical motions
        let mut motion1 = Motion::new(0.0f32).expect("finite initial value");
        let mut motion2 = Motion::new(0.0f32).expect("finite initial value");

        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        motion1
            .animate_to(100.0f32, config.clone())
            .expect("valid animation configuration");
        motion2
            .animate_to(100.0f32, config)
            .expect("valid animation configuration");

        // Run both animations and verify they produce identical results
        for step in 0..ANIMATION_STEPS {
            let result1 = motion1.update(DT).expect("representable animation frame");
            let result2 = motion2.update(DT).expect("representable animation frame");

            // Both should return the same continuation result
            assert_eq!(
                result1, result2,
                "Animation continuation mismatch at step {}",
                step
            );

            // Both should have the same current value (within floating point precision)
            let value_diff = (motion1.get_value() - motion2.get_value()).abs();
            assert!(
                value_diff < 0.001,
                "Animation values diverged at step {}: {} vs {}",
                step,
                motion1.get_value(),
                motion2.get_value()
            );

            // Both should have the same running state
            assert_eq!(
                motion1.is_running(),
                motion2.is_running(),
                "Running state mismatch at step {}",
                step
            );

            // If animation is complete, break
            if !result1 {
                break;
            }
        }

        // Final values should be identical
        assert_eq!(
            motion1.get_value(),
            motion2.get_value(),
            "Final animation values don't match"
        );
        assert_eq!(
            motion1.is_running(),
            motion2.is_running(),
            "Final running states don't match"
        );
    }

    /// Test motion memory usage efficiency
    #[test]
    fn test_motion_memory_efficiency() {
        use crate::Motion;
        use std::mem;

        let motion_size = mem::size_of::<Motion<f32>>();

        println!("Motion<f32> size: {} bytes", motion_size);

        // Total size should be reasonable
        assert!(
            motion_size <= 512,
            "Motion struct is too large: {} bytes",
            motion_size
        );
    }
}
