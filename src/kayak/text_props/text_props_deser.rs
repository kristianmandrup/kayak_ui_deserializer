use kayak_ui::{widgets::TextProps, prelude::{Alignment, KStyle}};

use crate::{ui_parser::Conv, serialized::STextProps, kayak::{store::KayakStore, alignment::to_alignment, kstyle::kstyle_deser::deserialize_kstyle}};

pub fn deserialize_text_props(store: &KayakStore, tp: STextProps) -> Result<TextProps, &'static str>  {
    TextPropsDeser::new(store, tp).deserialize()
}

pub struct TextPropsDeser<'a> {
    store: &'a KayakStore,
    node: STextProps
}
impl<'a> TextPropsDeser<'a> {
    pub fn new(store: &'a KayakStore, node: STextProps) -> Self {
        Self {
            store,
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

    fn user_styles(&self) -> Option<KStyle> {
        let prop = &self.node.user_styles.clone();
        deserialize_kstyle(prop.to_owned()).ok()
    }

    fn word_wrap(&self) -> Option<bool> {
        self.node.word_wrap.clone().unwrap().trim().parse().ok()
    }

    pub fn deserialize(&self) -> Result<TextProps, &'static str> {        
        let content = self.content();
        let font = self.font();
        let line_height = self.line_height();
        let show_cursor = self.show_cursor();
        let size = self.size();
        let alignment = self.alignment();
        let user_styles = self.user_styles();
        let word_wrap = self.word_wrap();
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
        if let Some(val) = user_styles {
            text_props.user_styles = val;    
        }
        if let Some(val) = word_wrap {
            text_props.word_wrap = val;    
        }
        Ok(text_props)            
    }

}