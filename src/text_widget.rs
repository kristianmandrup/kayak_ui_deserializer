pub struct UiTextWidget {
    node: TextWidget
}
impl UiTextWidget {
    fn parse(&self) -> Result<TextWidgetBundle, &'static str> {
        let text = UiTextProps::new(self.node).parse()?;
        if let Ok(content) = text.downcast::<TextProps>() {
            let widget = TextWidgetBundle {
                text: *content,
                ..Default::default()
            };        
            Ok(Box::new(widget))
        } else {
            Err("bad TextProps")
        }
    }
}

pub struct UiNode {
    pub width: Units
}
pub fn build_text_widget(ui: TextWidget) -> Result<UiNode, &'static str>  {
    if let Ok(unit) = UiUnit::new(ui.width).parse() {
        Ok(UiNode {
            width: unit
        })
    } else {
        Err("bad Text Widget")        
    }
    
}

