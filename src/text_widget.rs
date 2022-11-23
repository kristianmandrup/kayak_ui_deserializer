pub struct UiTextWidget {
    node: UiParseNode
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