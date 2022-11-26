use kayak_ui::widgets::TextBoxProps;

use crate::{json_deserializer::STextBoxProps, ui_parser::Conv};

pub struct TextBoxPropsBuilder {
    node: STextBoxProps
}
impl TextBoxPropsBuilder {
    pub fn new(node: STextBoxProps) -> Self {
        Self {
            node
        }
    }

    fn disabled(&self) -> Option<bool> {
        let prop = &&self.node.disabled.clone();
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_bool() 
        } else {
            None
        }
    }

    fn placeholder(&self) -> Option<String> {
        let prop = &self.node.placeholder.clone();
        Conv::get_prop(prop)
    }

    fn value(&self) -> Option<String> {
        let prop = &self.node.value.clone();
        Conv::get_prop(prop)
    }
}


impl TextBoxPropsBuilder {
    pub fn parse(&self) -> Result<TextBoxProps, &'static str> {        
        let disabled = self.disabled();
        let placeholder = self.placeholder();
        let value = self.value();
        let mut text_box_props = TextBoxProps::default();
        if let Some(val) = disabled {
            text_box_props.disabled = val;    
        }
        text_box_props.placeholder = placeholder;    
        if let Some(val) = value {
            text_box_props.value = val;    
        }
        Ok(text_box_props)            
    }
}