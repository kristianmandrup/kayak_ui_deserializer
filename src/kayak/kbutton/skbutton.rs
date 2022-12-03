use std::default;

use nanoserde::{DeJson, SerJson, DeRon, SerRon};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SKButton {
    // pub name: String,
    pub text: String,
    // pub styles: Option<SKStyle>,
}
impl Default for SKButton {
    fn default() -> SKButton {
        SKButton {
            text: "button".to_owned(),
        }
    }
}
