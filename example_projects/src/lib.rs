use dioxus::signals::{GlobalSignal, Signal};
use dioxus_motion::animations::colors::Color;

pub mod components;
pub static BG_COLOR: GlobalSignal<Color> = Signal::global(|| Color::from_rgba(59, 130, 246, 255));
