use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn KeyframeExample() -> Element {
    let (transform, mut animate_keyframes) = use_motion_store_with_keyframes(Transform::default());

    let start = move |_| {
        let animation = KeyframeAnimation::new(Duration::from_secs(2))
            .add_keyframe(
                Transform::new(0.0, 0.0, 1.0, 0.0),
                0.0,
                Some(Box::new(easer::functions::Cubic::ease_in)),
            )
            .add_keyframe(
                Transform::new(100.0, 0.0, 1.5, 45.0),
                0.3,
                Some(Box::new(easer::functions::Elastic::ease_out)),
            )
            .add_keyframe(
                Transform::new(100.0, 100.0, 0.8, 180.0),
                0.7,
                Some(Box::new(easer::functions::Bounce::ease_out)),
            )
            .add_keyframe(
                Transform::new(0.0, 0.0, 1.0, 360.0),
                1.0,
                Some(Box::new(easer::functions::Back::ease_in_out)),
            )
            .with_loop_mode(LoopMode::Alternate);

        animate_keyframes(animation);
    };

    rsx! {
        div {
            class: "demo-box",
            style: "transform: translate({}px, {}px) scale({}) rotate({}deg)",
            transform.current().x,
            transform.current().y,
            transform.current().scale,
            transform.current().rotation,
            onclick: start,
            "Click to animate"
        }
    }
}
