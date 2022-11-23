pub struct UiTextProps {
    node: UiParseNode
}
impl UiTextProps {
    fn new(node: UiParseNode) -> Self {
        Self {
            node
        }
    }

    fn content(&self) -> String {
        let prop = &self.node.text.clone();
        Conv::get_prop(prop)
    }

    fn font(&self) -> Option<String> {
        let prop = &self.node.font.clone();
        prop.to_owned()
    }

    fn line_height(&self) -> Option<f32> {
        let prop = &self.node.line_height.clone();
        Conv(Conv::get_prop(prop)).option_f32()
    }

    fn show_cursor(&self) -> bool {
        let prop = &self.node.show_cursor.clone();
        Conv(Conv::get_prop(prop)).to_bool() 
    }

    fn size(&self) -> f32 {
        let prop = &self.node.size.clone();
        Conv(Conv::get_prop(prop)).to_f32()
    }
    // alignment
    fn alignment(&self) -> Alignment {
        let prop = &self.node.size.clone();
        match prop {
            Some(align) => {
                to_alignment(align)
            },
            None => Alignment::Start
        }
    }
}

fn to_alignment(align: String) {
    match align.to_lowercase().as_str() {
        "start" => Alignment::Start,
        "middle" => Alignment::Middle,
        "end" => Alignment::End,
        _ => Alignment::Start
    }
}


impl UiParser for UiTextProps {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str> {        
        let font = self.font();
        let line_height = self.line_height();
        let show_cursor = self.show_cursor();
        let size = self.size();
        let alignment = self.alignment();
        let content = self.content();
        let widget = TextProps {
            content,
            font,      
            line_height,
            show_cursor,      
            size,
            alignment,
            ..Default::default()
        };
        Ok(Box::new(widget))            
    }

}