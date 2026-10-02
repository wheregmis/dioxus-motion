use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

const COLORS: [&str; 3] = ["#b9f078", "#6ee7b7", "#a5f3fc"];

/// One letter in the wave: a staggered sine loop bounces it while its
/// shadow squashes in sync.
#[component]
fn WaveLetter(letter: char, index: usize) -> Element {
    let mut transform = use_motion(Transform::identity())?;

    // Stagger only the first leg: a delayed Alternate loop replays the
    // delay on every leg and the letters fall out of sync.
    use_effect(move || {
        spawn(async move {
            if Time::delay(Duration::from_millis((index * 90) as u64))
                .await
                .is_err()
            {
                return;
            }
            transform
                .animate_to(
                    Transform {
                        y: -22.0,
                        scale: 1.15,
                        rotation: if index % 2 == 0 { -0.06 } else { 0.06 },
                        x: 0.0,
                    },
                    AnimationConfig::new(AnimationMode::Tween(Tween {
                        duration: Duration::from_millis(1100),
                        easing: easer::functions::Sine::ease_in_out,
                    }))
                    .with_loop(LoopMode::Alternate),
                )
                .expect("valid animation configuration");
        });
    });

    let t = transform.get_value();
    let squash = (1.0 + t.y / 44.0).max(0.4);

    rsx! {
        span { class: "inline-flex flex-col items-center w-10",
            span {
                class: "text-5xl font-black leading-none",
                style: "color: {COLORS[index % COLORS.len()]}; \
                       transform: translateY({t.y}px) scale({t.scale}) rotate({t.rotation}rad); \
                       text-shadow: 0 0 24px rgba(185,240,120,0.35);",
                "{letter}"
            }
            span {
                class: "w-6 h-1.5 rounded-full mt-2",
                style: "background: #ffffff10; \
                       transform: scaleX({squash}); \
                       opacity: {squash};",
            }
        }
    }
}

/// A wave of letters — each one is its own component with its own motion.
#[component]
pub fn BouncingText(text: &'static str) -> Element {
    rsx! {
        div { class: "flex items-end",
            for (index, letter) in text.chars().enumerate() {
                WaveLetter { key: "{index}", letter, index }
            }
        }
    }
}
