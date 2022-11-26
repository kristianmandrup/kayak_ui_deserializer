use kayak_ui::{widgets::{TextBoxBundle, TextBoxProps}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, ui_text_box::TextBoxPropsBuilder, serialized::STextBoxBundle};


pub fn build_text_box_bundle(bb: STextBoxBundle) -> Result<TextBoxBundle, &'static str>  {
    TextBoxBundleBuilder::new(bb).parse()
}

pub struct TextBoxBundleBuilder {
    node: STextBoxBundle,
}
impl TextBoxBundleBuilder {
    pub fn new(node: STextBoxBundle) -> Self {
        Self {
            node
        }
    }

    fn text_box(&self) -> Option<TextBoxProps> {
        let prop = &self.node.text_box.clone();
        if let Some(val) = prop {
            TextBoxPropsBuilder::new(val.to_owned()).parse().ok()
        } else {
            None
        }
        
    }

    fn style(&self) -> Option<KStyle> {
        let prop = &self.node.style.clone();
        if let Some(val) = prop {
            KStyleBuilder::new(val.to_owned()).parse().ok()
        } else {
            None
        }        
    }

    fn widget_name(&self) -> String {
        let prop = &self.node.name.clone();
        prop.to_owned()
    }

    pub fn parse(&self) -> Result<TextBoxBundle, &'static str> {                        
        let text_box = self.text_box();
        let styles = self.style();
        let name = self.widget_name();
        // let children = self.children();
        let mut text_box_bundle = TextBoxBundle::default();
        if let Some(val) = text_box {
            text_box_bundle.text_box = val;    
        }
        if let Some(val) = styles {
            text_box_bundle.styles = val;    
        }
        text_box_bundle.widget_name = WidgetName(name);            
        Ok(text_box_bundle)       
    }    
}