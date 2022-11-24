use kayak_ui::widgets::KButton;

use crate::{json_deserializer::Button, ui_style::StyleBuilder};

pub fn build_button(btn: Button) -> Result<KButton, &'static str>  {
    let mut button = KButton::default();
    let styles = StyleBuilder::new(btn.style).parse().unwrap();
    button.user_styles = styles;
    Ok(button)
}
