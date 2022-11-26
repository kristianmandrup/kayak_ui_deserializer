// use kayak_ui::widgets::{TextWidgetBundle, TextProps};

use kayak_ui::{widgets::{TextWidgetBundle}, prelude::WidgetName};

use crate::{ui_text_props::TextPropsBuilder, ui_kstyle::KStyleBuilder, serialized::STextWidgetBundle};

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

// TODO: builder
pub fn build_text_widget_bundle(tw: STextWidgetBundle) -> Result<TextWidgetBundle, &'static str>  {
    let text = TextPropsBuilder::new(tw.text).parse().unwrap();
    let styles = KStyleBuilder::new(tw.styles).parse().unwrap();
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

