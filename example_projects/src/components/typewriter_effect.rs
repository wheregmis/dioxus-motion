use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[component]
pub fn TypewriterEffect(text: &'static str) -> Element {
    let mut width =
        use_value_animation(Motion::new(0.0).to(100.0).duration(Duration::from_secs(2)));

    use_drop(move || {
        width.stop_loop();
    });

    rsx! {
        div {
            class: "font-mono text-2xl text-blue-500",
            style: "width: {width.value()}%;
                   white-space: nowrap;
                   overflow: hidden;
                   border-right: 2px solid currentColor;",
            onmounted: move |_| width.loop_animation(),
            "{text}"
        }
    }
}
