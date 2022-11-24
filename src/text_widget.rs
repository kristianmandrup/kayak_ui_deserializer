pub struct UiTextWidget {
    node: TextWidget
}
impl UiParser for UiTextWidget {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str> {
        let text = UiTextProps::new(self.node.to_owned()).parse()?;
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
impl UiNode {
    // fn new(width: UiNodeUnit) -> Self {
    //     Self {
    //         width
    //     }
    // }
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

