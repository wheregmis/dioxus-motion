use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

pub fn use_scroll_position() -> f64 {
    let mut scroll_y = use_signal(|| 0.0);

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let window_clone = window.clone();

        let closure = Closure::wrap(Box::new(move || {
            let new_scroll_y = window.scroll_y().unwrap_or(0.0);
            scroll_y.set(new_scroll_y);
        }) as Box<dyn FnMut()>);

        window_clone
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();

        // Initial scroll position
        let initial_scroll = window_clone.scroll_y().unwrap_or(0.0);
        scroll_y.set(initial_scroll);

        (move || {
            window_clone
                .remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();
        })()
    });

    let position = *scroll_y.read();
    position
}
