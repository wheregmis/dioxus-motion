/// Builds a presence style animation config from initial, animate, exit, and transition variants.
#[macro_export]
macro_rules! presence_style {
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        enter_transition: $enter_kind:ident { $($enter_transition:tt)* },
        exit_transition: $exit_kind:ident { $($exit_transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::with_transitions(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_transition!($enter_kind { $($enter_transition)* }),
            $crate::presence_style_transition!($exit_kind { $($exit_transition)* }),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        enter_transition: tween { $($enter_transition:tt)* },
        exit_transition: tween { $($exit_transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::with_transitions(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($enter_transition)*),
            $crate::presence_style_tween!($($exit_transition)*),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        enter_transition: spring { $($enter_transition:tt)* },
        exit_transition: spring { $($exit_transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::with_transitions(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($enter_transition)*),
            $crate::presence_style_spring!($($exit_transition)*),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        enter_transition: $enter_transition:expr,
        exit_transition: $exit_transition:expr $(,)?
    ) => {
        $crate::presence::PresenceConfig::with_transitions(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $enter_transition,
            $exit_transition,
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: tween { layout: tween { $($layout_transition:tt)* } $(, $transition_field:ident : $transition_value:expr)* $(,)? } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($transition_field : $transition_value),*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_tween!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: tween { layout: spring { $($layout_transition:tt)* } $(, $transition_field:ident : $transition_value:expr)* $(,)? } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($transition_field : $transition_value),*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_spring!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: spring { layout: tween { $($layout_transition:tt)* } $(, $transition_field:ident : $transition_value:expr)* $(,)? } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($transition_field : $transition_value),*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_tween!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: spring { layout: spring { $($layout_transition:tt)* } $(, $transition_field:ident : $transition_value:expr)* $(,)? } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($transition_field : $transition_value),*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_spring!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: tween { $($transition:tt)* },
        layout_transition: tween { $($layout_transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($transition)*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_tween!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: spring { $($transition:tt)* },
        layout_transition: spring { $($layout_transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($transition)*),
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($crate::presence_style_spring!($($layout_transition)*))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: tween { $($transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($transition)*),
        )
        .with_layout($crate::presence_style_layout!($layout))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: spring { $($transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($transition)*),
        )
        .with_layout($crate::presence_style_layout!($layout))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: $transition:expr,
        layout_transition: $layout_transition:expr $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $transition,
        )
        .with_layout($crate::presence_style_layout!($layout))
        .with_layout_transition($layout_transition)
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        layout: $layout:ident,
        transition: $transition:expr $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $transition,
        )
        .with_layout($crate::presence_style_layout!($layout))
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        transition: tween { $($transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_tween!($($transition)*),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        transition: spring { $($transition:tt)* } $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::presence_style_spring!($($transition)*),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        transition: tween_ms($milliseconds:expr) $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $crate::animations::core::AnimationConfig::tween_ms($milliseconds),
        )
    };
    (
        initial: { $($initial:tt)* },
        animate: { $($animate:tt)* },
        exit: { $($exit:tt)* },
        transition: $transition:expr $(,)?
    ) => {
        $crate::presence::PresenceConfig::new(
            $crate::motion_style!($($initial)*),
            $crate::motion_style!($($animate)*),
            $crate::motion_style!($($exit)*),
            $transition,
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_layout {
    (size) => {
        $crate::presence::PresenceLayout::Size
    };
    (none) => {
        $crate::presence::PresenceLayout::None
    };
    ($layout:ident) => {
        compile_error!(concat!(
            "unknown presence layout `",
            stringify!($layout),
            "`. Supported layouts are `size` and `none`."
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_transition {
    (tween { $($transition:tt)* }) => {
        $crate::presence_style_tween!($($transition)*)
    };
    (spring { $($transition:tt)* }) => {
        $crate::presence_style_spring!($($transition)*)
    };
    ($kind:ident { $($transition:tt)* }) => {
        compile_error!(concat!(
            "unknown transition kind `",
            stringify!($kind),
            "`. Supported transition kinds are `tween` and `spring`."
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_tween {
    () => {
        $crate::animations::core::AnimationConfig::new(
            $crate::animations::core::AnimationMode::Tween(
                $crate::animations::tween::Tween::default(),
            ),
        )
    };
    ($($field:ident : $value:expr),+ $(,)?) => {{
        let mut tween = $crate::animations::tween::Tween::default();
        $(
            $crate::presence_style_tween_assign!(tween, $field, $value);
        )+
        $crate::animations::core::AnimationConfig::new(
            $crate::animations::core::AnimationMode::Tween(tween),
        )
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_tween_assign {
    ($tween:ident, duration, $value:expr) => {
        $tween.duration = $crate::Duration::from_secs_f64(($value as f64) / 1000.0);
    };
    ($tween:ident, easing, $value:expr) => {
        $tween.easing = $value;
    };
    ($tween:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "unknown tween transition field `",
            stringify!($field),
            "`. Supported fields are `duration` and `easing`."
        ));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_spring {
    () => {
        $crate::animations::core::AnimationConfig::new(
            $crate::animations::core::AnimationMode::Spring(
                $crate::animations::spring::Spring::default(),
            ),
        )
    };
    ($($field:ident : $value:expr),+ $(,)?) => {{
        let mut spring = $crate::animations::spring::Spring::default();
        $(
            $crate::presence_style_spring_assign!(spring, $field, $value);
        )+
        $crate::animations::core::AnimationConfig::new(
            $crate::animations::core::AnimationMode::Spring(spring),
        )
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! presence_style_spring_assign {
    ($spring:ident, stiffness, $value:expr) => {
        $spring.stiffness = $value;
    };
    ($spring:ident, damping, $value:expr) => {
        $spring.damping = $value;
    };
    ($spring:ident, mass, $value:expr) => {
        $spring.mass = $value;
    };
    ($spring:ident, velocity, $value:expr) => {
        $spring.velocity = $value;
    };
    ($spring:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "unknown spring transition field `",
            stringify!($field),
            "`. Supported fields are `stiffness`, `damping`, `mass`, and `velocity`."
        ));
    };
}
