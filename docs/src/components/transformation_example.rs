use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
/// Renders a Dioxus component demonstrating animated transformations and glow effects on hover.
///
/// This component leverages Dioxus Motion to animate a card element. When hovered, the card smoothly translates, scales, and rotates using a spring-configured animation. On mouse leave, it reverts to its original state with a distinct spring animation. The component also computes dynamic CSS styles to apply both the transformation and a glow effect based on the current animation state.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use your_crate::TransformAnimationShowcase;
///
/// fn main() {
///     dioxus::desktop::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         TransformAnimationShowcase {}
///     })
/// }
/// ```
pub fn TransformAnimationShowcase() -> Element {
    let mut transform = use_motion(Transform::identity());

    let animate_hover = move |_| {
        transform.animate_to(
            Transform::new(
                0.0,                                  // x
                -20.0,                                // y
                1.1,                                  // scale
                5.0 * (std::f32::consts::PI / 180.0), // rotation in radians
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0, // Softer spring
                damping: 12.0,    // Less damping for bounce
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let animate_reset = move |_| {
        transform.animate_to(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 20.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let transform_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale({}) rotate({}deg);",
            transform.get_value().x,
            transform.get_value().y,
            transform.get_value().scale,
            transform.get_value().rotation * 180.0 / std::f32::consts::PI
        )
    });

    let glow_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale(1.2); opacity: {};",
            transform.get_value().x,
            transform.get_value().y,
            if transform.get_value().y < 0.0 {
                0.6
            } else {
                0.0
            }
        )
    });

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        div { class: "h-[400px] flex items-center justify-center p-4",
            div {
                class: "relative group cursor-pointer",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,
                // Main card - reduced from w-64/h-64 to w-48/h-48
                div {
                    class: "w-36 h-36 bg-gradient-to-tr from-emerald-400 to-cyan-400 rounded-xl shadow-xl",
                    style: "{transform_style.read()}",
                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-xl font-bold mb-1", "Hover Me!" }
                        span { class: "text-xs opacity-75", "Spring Animation" }
                    }
                }
                // Glow effect - scaled proportionally
                div {
                    class: "absolute inset-0 bg-gradient-to-tr from-emerald-400/30 to-cyan-400/30
                            rounded-2xl blur-lg -z-10",
                    style: "{glow_style.read()}",
                }
            }
        }
    }
}
