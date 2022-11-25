// use kayak_ui::widgets::{TextWidgetBundle, TextProps};

use kayak_ui::{widgets::{TextWidgetBundle}, prelude::WidgetName};

use crate::{json_deserializer::STextWidgetBundle, ui_text_props::UiTextProps, ui_kstyle::KStyleBuilder};

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
pub fn build_text_widget_bundle(tw: STextWidgetBundle) -> Result<TextWidgetBundle, &'static str>  {
    let text = UiTextProps::new(tw.text).parse().unwrap();
    let styles = KStyleBuilder::new(tw.style).parse().unwrap();
    let widget_name = WidgetName(tw.name);
    // pub styles: KStyle,
    // pub widget_name: WidgetName,        
    let text_widget_bundle = TextWidgetBundle {
        text,
        styles,
        widget_name
    };
    Ok(text_widget_bundle)
}

