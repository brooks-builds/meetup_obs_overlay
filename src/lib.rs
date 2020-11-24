use wasm_bindgen::prelude::*;
use web_sys::{console, window};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run(meetup_title: &str, meetup_speaker: &str) {
    init_panic_hook();
    let window = window().expect("could not find window");
    let document = window.document().unwrap();
    let meetup_title_element = document.query_selector("#meetup-title").unwrap().unwrap();
    meetup_title_element.set_inner_html(meetup_title);
    let meetup_speaker_element = document.query_selector("#talk").unwrap().unwrap();
    meetup_speaker_element.set_inner_html(meetup_speaker);
    console::log_1(&meetup_title_element);
}

#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(2 + 2, 4);
    }
}
