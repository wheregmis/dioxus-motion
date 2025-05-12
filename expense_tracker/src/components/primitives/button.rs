use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(into)]
    pub children: Element,

    #[props(default)]
    pub class: String,

    #[props(default)]
    pub variant: ButtonVariant,

    #[props(default)]
    pub size: ButtonSize,

    #[props(default)]
    pub disabled: bool,

    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    #[props(default)]
    pub motion: bool,

    #[props(default = "button".to_string())]
    pub button_type: String,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Outline,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_class = match props.variant {
        ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white",
        ButtonVariant::Secondary => "bg-gray-600 hover:bg-gray-700 text-white",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white",
        ButtonVariant::Success => "bg-green-600 hover:bg-green-700 text-white",
        ButtonVariant::Warning => "bg-yellow-600 hover:bg-yellow-700 text-white",
        ButtonVariant::Info => "bg-indigo-600 hover:bg-indigo-700 text-white",
        ButtonVariant::Outline => "bg-transparent border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
    };

    let size_class = match props.size {
        ButtonSize::Small => "px-2 py-1 text-sm",
        ButtonSize::Medium => "px-4 py-2",
        ButtonSize::Large => "px-6 py-3 text-lg",
    };

    let disabled_class = if props.disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };

    let class = format!(
        "font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 {} {} {} {}",
        base_class, size_class, disabled_class, props.class
    );

    if props.motion {
        let mut scale = use_motion(1.0f32);

        rsx! {
            button {
                class: "{class}",
                disabled: props.disabled,
                r#type: "{props.button_type}",
                onclick: move |e| {
                    tracing::info!("Button clicked (motion version), type: {}", props.button_type);
                    if let Some(onclick) = &props.onclick {
                        tracing::info!("Calling onclick handler");
                        onclick.call(e);
                    }
                },
                onmousedown: move |_| {
                    scale
                        .animate_to(
                            0.95,
                            AnimationConfig::new(
                                AnimationMode::Spring(Spring {
                                    stiffness: 300.0,
                                    damping: 15.0,
                                    mass: 0.2,
                                    velocity: 0.0,
                                }),
                            ),
                        );
                },
                onmouseup: move |_| {
                    scale
                        .animate_to(
                            1.0,
                            AnimationConfig::new(
                                AnimationMode::Spring(Spring {
                                    stiffness: 300.0,
                                    damping: 15.0,
                                    mass: 0.2,
                                    velocity: 0.0,
                                }),
                            ),
                        );
                },
                onmouseleave: move |_| {
                    scale
                        .animate_to(
                            1.0,
                            AnimationConfig::new(
                                AnimationMode::Spring(Spring {
                                    stiffness: 300.0,
                                    damping: 15.0,
                                    mass: 0.2,
                                    velocity: 0.0,
                                }),
                            ),
                        );
                },
                style: "transform: scale({scale.get_value()})",
                {props.children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{class}",
                disabled: props.disabled,
                r#type: "{props.button_type}",
                onclick: move |e| {
                    if let Some(onclick) = &props.onclick {
                        onclick.call(e);
                    }
                },
                {props.children}
            }
        }
    }
}
