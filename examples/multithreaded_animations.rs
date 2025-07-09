use dioxus::prelude::*;
use dioxus_motion::Duration;
use dioxus_motion::prelude::*;

fn main() {
    dioxus::launch(MultithreadedAnimationApp);
}

#[component]
fn MultithreadedAnimationApp() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-8",
            h1 { class: "text-3xl font-bold mb-8 text-center", "Multithreaded Animation Library Demo" }

            div { class: "max-w-6xl mx-auto space-y-8",
                // Basic multithreaded motion
                BasicMultithreadedDemo {}

                // Parallel processing demo
                ParallelProcessingDemo {}

                // Heavy computation demo
                HeavyComputationDemo {}

                // Scheduled animation demo
                ScheduledAnimationDemo {}

                // Performance comparison
                PerformanceComparisonDemo {}
            }
        }
    }
}

#[component]
fn BasicMultithreadedDemo() -> Element {
    let mut motion = use_motion_multithreaded(0.0f32);

    let animate = {
        let mut motion = motion.clone();
        move |_| {
            motion.animate_to(
                200.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        }
    };

    let animate_parallel = {
        let mut motion = motion.clone();
        move |_| {
            let targets = vec![
                (
                    100.0,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                ),
                (
                    150.0,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                ),
                (
                    200.0,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                ),
            ];
            motion.animate_to_parallel(targets);
        }
    };

    rsx! {
        section { class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold mb-4", "Basic Multithreaded Motion" }

            div { class: "space-y-4",
                div { class: "flex space-x-4",
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                        onclick: animate,
                        "Standard Animation"
                    }
                    button {
                        class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                        onclick: animate_parallel,
                        disabled: motion.is_processing(),
                        "Parallel Animation"
                    }
                }

                div { class: "text-sm text-gray-600",
                    p { "Value: {motion.get_value():.2}" }
                    p { "Processing: {motion.is_processing()}" }
                    p { "Running: {motion.is_running()}" }
                }

                div { class: "relative h-16 bg-gray-100 rounded overflow-hidden",
                    div {
                        class: "absolute w-4 h-4 bg-blue-500 rounded-full top-1/2 transform -translate-y-1/2 transition-all duration-100",
                        style: "left: {motion.get_value()}px;"
                    }
                }
            }
        }
    }
}

#[component]
fn ParallelProcessingDemo() -> Element {
    let mut batch_values = use_signal(|| vec![0.0f32; 20]);
    let mut is_processing = use_signal(|| false);

    let process_batch = move |_| {
        *is_processing.write() = true;

        spawn(async move {
            // Simulate processing 20 animations in parallel
            let springs_data: Vec<(f32, f32, Spring)> = (0..20)
                .map(|i| {
                    let current = batch_values.read()[i];
                    let target = (i as f32 * 20.0) + 100.0;
                    (current, target, Spring::default())
                })
                .collect();

            let results =
                ParallelAnimationProcessor::process_springs_parallel(springs_data, 0.016).await;

            *batch_values.write() = results;
            *is_processing.write() = false;
        });
    };

    // Fix temporary value issue by creating a local binding
    let values = batch_values();
    let values_iter = values.iter().enumerate().collect::<Vec<_>>();

    rsx! {
        section { class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold mb-4", "Parallel Processing Demo" }

            div { class: "space-y-4",
                button {
                    class: "px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600",
                    onclick: process_batch,
                    disabled: is_processing(),
                    if is_processing() { "Processing..." } else { "Process 20 Animations in Parallel" }
                }

                div { class: "grid grid-cols-10 gap-2",
                    {values_iter.into_iter().map(|(i, &value)| {
                        rsx! {
                            div {
                                key: "{i}",
                                class: "w-8 h-8 bg-purple-300 rounded flex items-center justify-center text-xs font-bold",
                                style: "transform: translateY({value * 0.5}px)",
                                "{i}"
                            }
                        }
                    })}
                }

                p { class: "text-sm text-gray-600",
                    "This demonstrates parallel spring physics calculations for multiple elements simultaneously."
                }
            }
        }
    }
}

#[component]
fn HeavyComputationDemo() -> Element {
    let mut motion = use_motion_multithreaded(Transform::default());
    let mut computation_time = use_signal(|| 0.0f32);

    let start_heavy_computation = {
        let mut motion_clone = motion.clone();
        move |_| {
            let start_time = instant::Instant::now();

            motion_clone.animate_to_heavy(
                Transform::new(300.0, 200.0, 1.5, 180.0),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

            // Create a separate clone for the async block
            let mut monitor_motion = motion_clone.clone();
            spawn(async move {
                // Simulate monitoring computation time
                while monitor_motion.is_processing() {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    *computation_time.write() = elapsed;
                    async_std::task::sleep(Duration::from_millis(10)).await;
                }
            });
        }
    };

    rsx! {
        section { class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold mb-4", "Heavy Computation Demo" }

            div { class: "space-y-4",
                button {
                    class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                    onclick: start_heavy_computation,
                    disabled: motion.is_processing(),
                    "Start Heavy Computation Animation"
                }

                div { class: "text-sm text-gray-600",
                    p { "Computation Time: {computation_time():.3}s" }
                    p { "Processing: {motion.is_processing()}" }
                    p { "X: {motion.get_value().x:.1}, Y: {motion.get_value().y:.1}" }
                }

                div { class: "relative h-32 bg-gray-100 rounded overflow-hidden",
                    div {
                        class: "absolute w-8 h-8 bg-red-500 rounded transition-all duration-100",
                        style: "transform: translate({motion.get_value().x}px, {motion.get_value().y}px) scale({motion.get_value().scale}) rotate({motion.get_value().rotation}deg);"
                    }
                }

                p { class: "text-sm text-gray-600",
                    "This demonstrates offloading expensive calculations to separate threads using spawn."
                }
            }
        }
    }
}

#[component]
fn ScheduledAnimationDemo() -> Element {
    let mut scheduled_motion = use_motion_scheduled(0.0f32);
    let mut animation_count = use_signal(|| 1u32);

    let add_scheduled_animation = {
        let mut scheduled_motion = scheduled_motion.clone();
        move |_| {
            let count = *animation_count.read();
            *animation_count.write() = count + 1;

            // Create multiple scheduled animations
            for i in 0..count {
                let target = (i as f32 * 50.0) + 100.0;
                scheduled_motion.animate_to(
                    target,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                );
            }
        }
    };

    rsx! {
        section { class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold mb-4", "Scheduled Animation Demo" }

            div { class: "space-y-4",
                div { class: "flex items-center space-x-4",
                    button {
                        class: "px-4 py-2 bg-indigo-500 text-white rounded hover:bg-indigo-600",
                        onclick: add_scheduled_animation,
                        "Add Scheduled Animation"
                    }

                    div { class: "flex items-center space-x-2",
                        label { "Count:" }
                        input {
                            r#type: "range",
                            min: "1",
                            max: "10",
                            value: "{animation_count()}",
                            oninput: move |e| {
                                if let Ok(val) = e.value().parse::<u32>() {
                                    *animation_count.write() = val;
                                }
                            }
                        }
                        span { "{animation_count()}" }
                    }
                }

                div { class: "text-sm text-gray-600",
                    p { "Value: {scheduled_motion.get_value():.2}" }
                    p { "Running: {scheduled_motion.is_running()}" }
                }

                div { class: "relative h-16 bg-gray-100 rounded overflow-hidden",
                    div {
                        class: "absolute w-4 h-4 bg-indigo-500 rounded-full top-1/2 transform -translate-y-1/2 transition-all duration-100",
                        style: "left: {scheduled_motion.get_value()}px;"
                    }
                }

                p { class: "text-sm text-gray-600",
                    "This demonstrates the centralized animation scheduler processing multiple animations efficiently."
                }
            }
        }
    }
}

#[component]
fn PerformanceComparisonDemo() -> Element {
    let mut standard_motion = use_motion(0.0f32);
    let mut multithreaded_motion = use_motion_multithreaded(0.0f32);
    let mut performance_data = use_signal(|| Vec::<(String, f32)>::new());

    let run_performance_test = {
        let mut standard_motion_clone = standard_motion.clone();
        let mut multithreaded_motion_clone = multithreaded_motion.clone();
        move |_| {
            // Create separate clones for the async block
            let mut std_motion = standard_motion_clone.clone();
            let mut mt_motion = multithreaded_motion_clone.clone();

            spawn(async move {
                let mut results = Vec::new();

                // Test standard animation
                let start = instant::Instant::now();
                for i in 0..100 {
                    std_motion.animate_to(
                        i as f32 * 2.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                    );
                    async_std::task::sleep(Duration::from_millis(1)).await;
                }
                let standard_time = start.elapsed().as_secs_f32();
                results.push(("Standard".to_string(), standard_time));

                // Test multithreaded animation
                let start = instant::Instant::now();
                let targets: Vec<_> = (0..100)
                    .map(|i| {
                        (
                            i as f32 * 2.0,
                            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                        )
                    })
                    .collect();
                mt_motion.animate_to_parallel(targets);

                while mt_motion.is_processing() {
                    async_std::task::sleep(Duration::from_millis(10)).await;
                }
                let multithreaded_time = start.elapsed().as_secs_f32();
                results.push(("Multithreaded".to_string(), multithreaded_time));

                *performance_data.write() = results;
            });
        }
    };

    // Fix temporary value issue
    let perf_data = performance_data();
    let perf_iter = perf_data.iter().collect::<Vec<_>>();

    rsx! {
        section { class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-xl font-semibold mb-4", "Performance Comparison" }

            div { class: "space-y-4",
                button {
                    class: "px-4 py-2 bg-yellow-500 text-white rounded hover:bg-yellow-600",
                    onclick: run_performance_test,
                    "Run Performance Test"
                }

                if !perf_data.is_empty() {
                    div { class: "space-y-2",
                        h3 { class: "font-semibold", "Results (100 animations):" }
                        {perf_iter.into_iter().map(|(name, time)| {
                            rsx! {
                                div {
                                    key: "{name}",
                                    class: "flex justify-between items-center p-2 bg-gray-100 rounded",
                                    span { "{name}" }
                                    span { class: "font-mono", "{time:.3}s" }
                                }
                            }
                        })}
                    }
                }

                p { class: "text-sm text-gray-600",
                    "This compares the performance of standard vs multithreaded animation processing."
                }
            }
        }
    }
}
