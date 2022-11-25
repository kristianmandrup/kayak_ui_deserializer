use kayak_ui::widgets::KButton;

use crate::{json_deserializer::SButton, ui_style::StyleBuilder};

// pub struct KButtonBundle {
//     pub button: KButton,
//     pub styles: KStyle,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }


pub fn build_button(btn: SButton) -> Result<KButton, &'static str>  {
    let mut button = KButton::default();
    let styles = StyleBuilder::new(btn.style).parse().unwrap();
    button.user_styles = styles;
    Ok(button)
}