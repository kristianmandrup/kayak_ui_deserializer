use kayak_ui::{widgets::TextProps, prelude::Alignment};

use crate::{json_deserializer::Text, ui_alignment::to_alignment, ui_parser::Conv};

pub struct UiTextProps {
    node: Text
}
impl UiTextProps {
    fn new(node: Text) -> Self {
        Self {
            node
        }
    }

    fn content(&self) -> Option<String> {
        let prop = &self.node.content.clone();
        Conv::get_prop(prop)
    }

    fn font(&self) -> Option<String> {
        let prop = &self.node.font.clone();
        prop.to_owned()
    }

    fn line_height(&self) -> Option<f32> {
        let prop = &self.node.line_height.clone();
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).option_f32()
        } else {
            None
        }
        
    }

    fn show_cursor(&self) -> Option<bool> {
        let prop = &self.node.show_cursor.clone();
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_bool() 
        } else {
            None
        }
    }

    fn size(&self) -> Option<f32> {
        let prop = &self.node.size.clone();
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32() 
        } else {
            None
        }
    }
    // alignment
    fn alignment(&self) -> Option<Alignment> {
        let prop = self.node.alignment.clone();
        if let Some(val) = prop {
            Some(to_alignment(val))
        } else {
            None
        }
    }
}


impl UiTextProps {
    fn parse(&self) -> Result<TextProps, &'static str> {        
        let content = self.content();
        let font = self.font();
        let line_height = self.line_height();
        let show_cursor = self.show_cursor();
        let size = self.size();
        let alignment = self.alignment();
        let mut text_props = TextProps::default();
        if let Some(val) = content.clone() {
            text_props.content = val;    
        }

        text_props.font = font; 
        text_props.line_height = line_height;    

        if let Some(val) = show_cursor {
            text_props.show_cursor = val;    
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