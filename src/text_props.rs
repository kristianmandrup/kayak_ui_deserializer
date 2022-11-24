pub struct UiTextProps {
    node: UiParseNode
}
impl UiTextProps {
    fn new(node: UiParseNode) -> Self {
        Self {
            node
        }
    }

    fn content(&self) -> Option<String> {
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


impl UiTextProps {
    fn parse(&self) -> Result<TextProps, &'static str> {        
        let font = self.font();
        let line_height = self.line_height();
        let show_cursor = self.show_cursor();
        let size = self.size();
        let alignment = self.alignment();
        let content = self.content();
        let mut text_props = TextProps::default();
        if let Some(val) = content {
            text_props.content = val;    
        }
        if let Some(val) = content {
            text_props.content = val;    
        }
        if let Some(val) = font {
            text_props.font = val;    
        }
        if let Some(val) = line_height {
            text_props.line_height = val;    
        }
        if let Some(val) = show_cursor {
            text_props.content = val;    
        }
        if let Some(val) = size {
            text_props.size = val;    
        }
        if let Some(val) = alignment {
            text_props.alignment = val;    
        }
        Ok(text_props)            
    }

}