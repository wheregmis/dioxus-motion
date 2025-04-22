use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let boxes = (0..5).map(|i| {
        let delay = i as f32 * 0.1;
        rsx! {
            motion::div {
                key: "{i}",
                class: "box small",
                initial: AnimationTarget::new().y(50.0).opacity(0.0),
                animate: AnimationTarget::new().y(0.0).opacity(1.0),
                transition: TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(10.0)
                    .delay(delay),
                while_hover: AnimationTarget::new().scale(1.1).rotate(5.0),
                "{i + 1}"
            }
        }
    });

    rsx! {
        div { class: "container",
            h1 { "Motion Components Example" }

            // Basic motion div with hover animation
            motion::div {
                class: "box",
                animate: AnimationTarget::new().x(0.0).opacity(1.0),
                initial: AnimationTarget::new().x(-100.0).opacity(0.0),
                transition: TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(10.0),
                while_hover: AnimationTarget::new().scale(1.2),
                "Hover me!"
            }

            // Button with tap animation
            motion::button {
                class: "button",
                animate: AnimationTarget::new().background_color("#2ecc71"),
                while_tap: AnimationTarget::new().scale(0.9),
                while_hover: AnimationTarget::new().background_color("#0088ff"),
                transition: TransitionConfig::new(TransitionType::Spring)
                    .stiffness(300.0)
                    .damping(20.0),
                "Click me!"
            }

            // Sequence of boxes with staggered animations
            div { class: "boxes",
                {boxes}
            }
        }

        // Add some basic styles
        style { {r#"
            .container {
                max-width: 800px;
                margin: 0 auto;
                padding: 2rem;
                font-family: system-ui, sans-serif;
            }

            h1 {
                margin-bottom: 2rem;
            }

            .box {
                width: 200px;
                height: 100px;
                background-color: #3498db;
                color: white;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 8px;
                margin-bottom: 1rem;
                font-weight: bold;
                cursor: pointer;
            }

            .small {
                width: 60px;
                height: 60px;
                margin-right: 1rem;
            }

            .boxes {
                display: flex;
                margin-top: 2rem;
            }

            .button {
                padding: 0.75rem 1.5rem;
                background-color: #2ecc71;
                color: white;
                border: none;
                border-radius: 4px;
                font-weight: bold;
                cursor: pointer;
            }
        "#} }
    }
}
