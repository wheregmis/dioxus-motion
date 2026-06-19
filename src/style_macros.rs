/// Builds a [`MotionStyle`](crate::animations::style::MotionStyle) value from a CSS-like property block.
///
/// Use with [`use_motion`](crate::use_motion) to animate CSS properties on always-mounted elements,
/// or compose three calls into `presence_style!` for enter/exit animations.
#[macro_export]
macro_rules! motion_style {
    () => {
        $crate::animations::style::MotionStyle::default()
    };
    ($($field:ident : $value:expr),+ $(,)?) => {{
        let mut style = $crate::animations::style::MotionStyle::default();
        $(
            $crate::motion_style_assign!(style, $field, $value);
        )+
        style
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! motion_style_assign {
    ($style:ident, opacity, $value:expr) => {
        $style.opacity = ($value) as f32;
    };
    ($style:ident, x, $value:expr) => {
        $style.x = ($value) as f32;
    };
    ($style:ident, y, $value:expr) => {
        $style.y = ($value) as f32;
    };
    ($style:ident, z, $value:expr) => {
        $style.z = ($value) as f32;
    };
    ($style:ident, translateX, $value:expr) => {
        $style.x = ($value) as f32;
    };
    ($style:ident, translate_x, $value:expr) => {
        $style.x = ($value) as f32;
    };
    ($style:ident, translateY, $value:expr) => {
        $style.y = ($value) as f32;
    };
    ($style:ident, translate_y, $value:expr) => {
        $style.y = ($value) as f32;
    };
    ($style:ident, translateZ, $value:expr) => {
        $style.z = ($value) as f32;
    };
    ($style:ident, translate_z, $value:expr) => {
        $style.z = ($value) as f32;
    };
    ($style:ident, scale, $value:expr) => {
        $style.scale = ($value) as f32;
    };
    ($style:ident, scaleX, $value:expr) => {
        $style.scale_x = ($value) as f32;
    };
    ($style:ident, scale_x, $value:expr) => {
        $style.scale_x = ($value) as f32;
    };
    ($style:ident, scaleY, $value:expr) => {
        $style.scale_y = ($value) as f32;
    };
    ($style:ident, scale_y, $value:expr) => {
        $style.scale_y = ($value) as f32;
    };
    ($style:ident, scaleZ, $value:expr) => {
        $style.scale_z = ($value) as f32;
    };
    ($style:ident, scale_z, $value:expr) => {
        $style.scale_z = ($value) as f32;
    };
    ($style:ident, rotate, $value:expr) => {
        $style.rotate = ($value) as f32;
    };
    ($style:ident, rotateX, $value:expr) => {
        $style.rotate_x = ($value) as f32;
    };
    ($style:ident, rotate_x, $value:expr) => {
        $style.rotate_x = ($value) as f32;
    };
    ($style:ident, rotateY, $value:expr) => {
        $style.rotate_y = ($value) as f32;
    };
    ($style:ident, rotate_y, $value:expr) => {
        $style.rotate_y = ($value) as f32;
    };
    ($style:ident, rotateZ, $value:expr) => {
        $style.rotate_z = ($value) as f32;
    };
    ($style:ident, rotate_z, $value:expr) => {
        $style.rotate_z = ($value) as f32;
    };
    ($style:ident, skew, $value:expr) => {
        $style.skew = ($value) as f32;
    };
    ($style:ident, skewX, $value:expr) => {
        $style.skew_x = ($value) as f32;
    };
    ($style:ident, skew_x, $value:expr) => {
        $style.skew_x = ($value) as f32;
    };
    ($style:ident, skewY, $value:expr) => {
        $style.skew_y = ($value) as f32;
    };
    ($style:ident, skew_y, $value:expr) => {
        $style.skew_y = ($value) as f32;
    };
    ($style:ident, perspective, $value:expr) => {
        $style.perspective = ($value) as f32;
    };
    ($style:ident, transformPerspective, $value:expr) => {
        $style.perspective = ($value) as f32;
    };
    ($style:ident, transform_perspective, $value:expr) => {
        $style.perspective = ($value) as f32;
    };
    ($style:ident, rotation, $value:expr) => {
        compile_error!("use `rotate` instead of `rotation`; `rotate` is expressed in degrees");
    };
    ($style:ident, $field:ident, $value:expr) => {
        $style.add_css_property(stringify!($field), $value);
    };
}
