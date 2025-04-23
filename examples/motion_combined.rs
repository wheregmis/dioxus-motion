use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use std::collections::HashMap;

// Configure different transition settings based on platform
#[cfg(feature = "web")]
const STIFFNESS: f32 = 200.0;
#[cfg(feature = "web")]
const DAMPING: f32 = 20.0;

#[cfg(not(feature = "web"))]
const STIFFNESS: f32 = 300.0;
#[cfg(not(feature = "web"))]
const DAMPING: f32 = 30.0;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut show_card = use_signal(|| false);
    let mut show_components = use_signal(|| true);

    // Define variants for the card
    let card_variants = {
        let mut variants = HashMap::new();
        variants.insert(
            "hidden".to_string(),
            AnimationTarget::new().opacity(0.0).y(50.0).scale(0.8),
        );
        variants.insert(
            "visible".to_string(),
            AnimationTarget::new().opacity(1.0).y(0.0).scale(1.0),
        );
        variants
    };

    // Define variants for the items
    let item_variants = {
        let mut variants = HashMap::new();
        variants.insert(
            "hidden".to_string(),
            AnimationTarget::new().opacity(0.0).x(-20.0),
        );
        variants.insert(
            "visible".to_string(),
            AnimationTarget::new().opacity(1.0).x(0.0),
        );
        variants
    };

    // Create staggered boxes for the components section
    let boxes = (0..5).map(|i| {
        let delay = i as f32 * 0.1;
        let box_number = i + 1;
        rsx! {
            motion::div {
                key: "box-{box_number}",
                class: "box small",
                initial: Some(AnimationTarget::new().y(50.0).opacity(0.0)),
                animate: Some(AnimationTarget::new().y(0.0).opacity(1.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(STIFFNESS)
                        .damping(DAMPING)
                        .delay(delay),
                ),
                while_hover: Some(AnimationTarget::new().scale(1.1).rotate(5.0)),
                {box_number.to_string()}
            }
        }
    });

    rsx! {
        div { class: "container",
            h1 { "Dioxus Motion Showcase" }

            // Section toggle buttons
            div { class: "section-toggles",
                button {
                    class: if show_components() { "toggle-button active" } else { "toggle-button" },
                    onclick: move |_| show_components.set(true),
                    "Motion Components"
                }
                button {
                    class: if !show_components() { "toggle-button active" } else { "toggle-button" },
                    onclick: move |_| show_components.set(false),
                    "Motion Variants"
                }
            }

            // Motion Components Section
            div {
                class: if show_components() { "section active" } else { "section" },
                h2 { "Motion Components Example" }

                // Basic motion div with hover animation
                motion::div {
                    class: "box",
                    animate: Some(AnimationTarget::new().x(0.0).opacity(1.0)),
                    initial: Some(AnimationTarget::new().x(-100.0).opacity(0.0)),
                    transition: Some(TransitionConfig::new(TransitionType::Spring)
                        .stiffness(STIFFNESS)
                        .damping(DAMPING)),
                    while_hover: Some(AnimationTarget::new().scale(1.2)),
                    "Hover me!"
                }

                // Button with tap animation
                motion::button {
                    class: "button",
                    animate: Some(AnimationTarget::new().background_color("#2ecc71")),
                    while_tap: Some(AnimationTarget::new().scale(0.9)),
                    while_hover: Some(AnimationTarget::new().background_color("#0088ff")),
                    transition: Some(TransitionConfig::new(TransitionType::Spring)
                        .stiffness(STIFFNESS * 1.5)
                        .damping(DAMPING)),
                    "Click me!"
                }

                // Sequence of boxes with staggered animations
                div { class: "boxes", {boxes} }
            }

            // Motion Variants Section
            div {
                class: if !show_components() { "section active" } else { "section" },
                h2 { "Motion Variants Example" }

                // Toggle button
                button {
                    class: "toggle-button",
                    onclick: move |_| {
                        // Explicitly set the value instead of using toggle
                        let new_value = !show_card();
                        show_card.set(new_value);
                        println!("Card visibility toggled to: {}", new_value);
                    },
                    if show_card() {
                        "Hide Card"
                    } else {
                        "Show Card"
                    }
                }

                // Card with variants
                motion::div {
                    class: "card",
                    variants: Some(card_variants.clone()),
                    initial_variant: Some("hidden".to_string()),
                    animate_variant: Some(if show_card() { "visible" } else { "hidden" }.to_string()),
                    transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(STIFFNESS).damping(DAMPING)),

                    // Title
                    div { class: "card-title",
                        motion::div {
                            variants: Some(item_variants.clone()),
                            initial_variant: Some("hidden".to_string()),
                            animate_variant: Some(if show_card() { "visible" } else { "hidden" }.to_string()),
                            transition: Some(
                                TransitionConfig::new(TransitionType::Spring)
                                    .stiffness(STIFFNESS)
                                    .damping(DAMPING)
                                    .delay(0.05),
                            ),
                            h2 { "Card Title" }
                        }
                    }

                    // Description
                    div { class: "card-description",
                        motion::div {
                            variants: Some(item_variants.clone()),
                            initial_variant: Some("hidden".to_string()),
                            animate_variant: Some(if show_card() { "visible" } else { "hidden" }.to_string()),
                            transition: Some(
                                TransitionConfig::new(TransitionType::Spring)
                                    .stiffness(STIFFNESS)
                                    .damping(DAMPING)
                                    .delay(0.1),
                            ),
                            p {
                                "This is a card with animated content using variants. Each element animates with a staggered delay."
                            }
                        }
                    }

                    // Button
                    motion::button {
                        class: "card-button",
                        variants: Some(item_variants.clone()),
                        initial_variant: Some("hidden".to_string()),
                        animate_variant: Some(if show_card() { "visible" } else { "hidden" }.to_string()),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(STIFFNESS)
                                .damping(DAMPING)
                                .delay(0.15),
                        ),
                        while_hover: Some(AnimationTarget::new().scale(1.05)),
                        while_tap: Some(AnimationTarget::new().scale(0.95)),
                        "Learn More"
                    }
                }
            }

            // Add some basic styles
            style {
                ".container {{ max-width: 800px; margin: 0 auto; padding: 2rem; font-family: system-ui, sans-serif; }}
                h1 {{ margin-bottom: 1rem; text-align: center; }}
                h2 {{ margin-bottom: 1.5rem; }}
                .section-toggles {{ display: flex; justify-content: center; margin-bottom: 2rem; gap: 1rem; }}
                .toggle-button {{ padding: 0.75rem 1.5rem; background-color: #3498db; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer; }}
                .toggle-button.active {{ background-color: #2980b9; }}
                .section {{ display: none; padding: 1rem; border-radius: 8px; background-color: #f9f9f9; }}
                .section.active {{ display: block; }}
                .box {{ width: 200px; height: 100px; background-color: #3498db; color: white; display: flex; align-items: center; justify-content: center; border-radius: 8px; margin-bottom: 1rem; font-weight: bold; cursor: pointer; }}
                .small {{ width: 60px; height: 60px; margin-right: 1rem; }}
                .boxes {{ display: flex; margin-top: 2rem; margin-bottom: 2rem; }}
                .button {{ padding: 0.75rem 1.5rem; background-color: #2ecc71; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer; margin-top: 1rem; }}
                .card {{ background-color: white; border-radius: 8px; padding: 2rem; box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1); max-width: 500px; margin-top: 2rem; }}
                .card-title {{ margin-top: 0; margin-bottom: 1rem; color: #333; }}
                .card-description {{ color: #666; line-height: 1.6; margin-bottom: 1.5rem; }}
                .card-button {{ padding: 0.75rem 1.5rem; background-color: #2ecc71; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer; }}"
            }
        }
    }
}
