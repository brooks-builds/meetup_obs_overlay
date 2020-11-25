use std::{thread::sleep, time::Duration};

use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;
use web_sys::{console, window, Element};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// #[wasm_bindgen]
// pub fn run(meetup_title: &str, meetup_speaker: &str) {
//     init_panic_hook();
//     let window = window().expect("could not find window");
//     let document = window.document().unwrap();
//     let meetup_title_element = document.query_selector("#meetup-title").unwrap().unwrap();
//     meetup_title_element.set_inner_html(meetup_title);
//     let meetup_speaker_element = document.query_selector("#talk").unwrap().unwrap();
//     meetup_speaker_element.set_inner_html(meetup_speaker);
//     let other_info_element = document.query_selector("#other").unwrap().unwrap();
//     // other_info_element.set_class_name(&format!("popup {}", classes));
//     add_class("popup", other_info_element);
//     sleep(Duration::from_secs(5));
//     remove_class();
// }

// #[wasm_bindgen]
// pub fn add_class(name: &str, element: Element) {
//     let classes = format!("{} {}", name, element.class_name());
//     element.set_class_name(&classes);
// }

// #[wasm_bindgen]
// pub fn remove_class() {
//     let window = window().expect("could not find window");
//     let document = window.document().unwrap();
//     console::log_1(&"test".into());
// }

#[wasm_bindgen]
#[repr(C)]
pub struct App {
    title: String,
    speaker: String,
    info: Vec<String>,
}

#[wasm_bindgen]
impl App {
    pub fn new() -> Self {
        Self {
            title: "Meetup".to_owned(),
            speaker: "Brooks Patton".to_owned(),
            info: vec![],
        }
    }

    pub fn add_information(&mut self, info: &str) {
        self.info.push(info.to_owned());
    }

    pub fn add_class(&self, class_name: &str, id: &str) {
        let window = window().expect("could not find window");
        let document = window.document().unwrap();
        let element = document.query_selector(id).unwrap().unwrap();
        let classes = Array::new();
        classes.push(&class_name.into());
        element.class_list().add(&classes).unwrap();
    }

    pub fn remove_class(&self, class_name: &str, id: &str) {
        let window = window().expect("could not find window");
        let document = window.document().unwrap();
        let element = document.query_selector(id).unwrap().unwrap();
        let classes = Array::new();
        classes.push(&class_name.into());
        element.class_list().remove(&classes).unwrap();
    }

    pub fn set_speaker(&self) {
        let window = window().expect("could not find window");
        let document = window.document().unwrap();
        let speaker_element = document.query_selector("#speaker").unwrap().unwrap();
        speaker_element.set_inner_html(&self.speaker);
    }

    pub fn set_title(&self) {
        let window = window().expect("could not find window");
        let document = window.document().unwrap();
        let meetup_title_element = document.query_selector("#meetup-title").unwrap().unwrap();
        meetup_title_element.set_inner_html(&self.title);
    }

    pub fn with_speaker(mut self, speaker: &str) -> Self {
        self.speaker = speaker.to_owned();
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canary_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_initializing() {
        let app = App::new();

        assert_eq!(app.title, "Meetup");
        assert_eq!(app.speaker, "Brooks Patton");
        assert_eq!(app.info, Vec::<String>::new());
    }

    #[test]
    fn test_overriding_defaults() {
        let title = "Denver Rust Meetup";
        let speaker = "Another Person";
        let infos = vec!["a cool thing", "more information!"];
        let mut app = App::new().with_title(title).with_speaker(speaker);

        infos.iter().for_each(|info| app.add_information(info));

        assert_eq!(app.title, title);
        assert_eq!(app.speaker, speaker);
        assert_eq!(app.info[0], infos[0]);
        assert_eq!(app.info[1], infos[1]);
    }
}
