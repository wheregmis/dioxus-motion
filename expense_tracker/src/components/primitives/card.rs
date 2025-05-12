use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(into)]
    pub children: Element,
    
    #[props(default)]
    pub class: String,
    
    #[props(default)]
    pub motion: bool,
    
    #[props(default)]
    pub hover_effect: bool,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_class = "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden";
    
    let hover_class = if props.hover_effect {
        "hover:shadow-md transition-shadow"
    } else {
        ""
    };
    
    let class = format!("{} {} {}", base_class, hover_class, props.class);
    
    if props.motion {
        let mut scale = use_motion(1.0f32);
        let mut y_offset = use_motion(0.0f32);
        
        rsx! {
            div {
                class: "{class}",
                style: "transform: scale({scale.get_value()}) translateY({y_offset.get_value()}px)",
                onmouseenter: move |_| {
                    scale.animate_to(
                        1.02,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 300.0,
                            damping: 15.0,
                            mass: 0.2,
                            velocity: 0.0,
                        }))
                    );
                    
                    y_offset.animate_to(
                        -5.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 300.0,
                            damping: 15.0,
                            mass: 0.2,
                            velocity: 0.0,
                        }))
                    );
                },
                onmouseleave: move |_| {
                    scale.animate_to(
                        1.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 300.0,
                            damping: 15.0,
                            mass: 0.2,
                            velocity: 0.0,
                        }))
                    );
                    
                    y_offset.animate_to(
                        0.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 300.0,
                            damping: 15.0,
                            mass: 0.2,
                            velocity: 0.0,
                        }))
                    );
                },
                
                {props.children}
            }
        }
    } else {
        rsx! {
            div {
                class: "{class}",
                
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(into)]
    pub children: Element,
    
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let class = format!("p-4 border-b border-gray-200 dark:border-gray-700 {}", props.class);
    
    rsx! {
        div {
            class: "{class}",
            
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    #[props(into)]
    pub children: Element,
    
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    let class = format!("p-4 {}", props.class);
    
    rsx! {
        div {
            class: "{class}",
            
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    #[props(into)]
    pub children: Element,
    
    #[props(default)]
    pub class: String,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let class = format!("p-4 border-t border-gray-200 dark:border-gray-700 {}", props.class);
    
    rsx! {
        div {
            class: "{class}",
            
            {props.children}
        }
    }
}
