// use kayak_ui::widgets::{TextWidgetBundle, TextProps};

// use crate::json_deserializer::TextWidget;

// pub struct UiTextWidget {
//     node: TextWidget
// }
// impl UiTextWidget {
//     fn parse(&self) -> Result<TextWidgetBundle, &'static str> {
//         let text = UiTextProps::new(self.node).parse()?;
//         if let Ok(content) = text.downcast::<TextProps>() {
//             let widget = TextWidgetBundle {
//                 text: *content,
//                 ..Default::default()
//             };        
//             Ok(widget)
//         } else {
//             Err("bad TextProps")
//         }
//     }
// }

// pub struct UiNode {
//     pub width: Units
// }
// pub fn build_text_widget(ui: TextWidget) -> Result<UiNode, &'static str>  {
    
// }

