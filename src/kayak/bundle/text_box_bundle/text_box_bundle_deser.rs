use kayak_ui::{widgets::{TextBoxBundle, TextBoxProps}, prelude::{KStyle, WidgetName}};

use crate::{kayak::{store::KayakStore, text_box_props::text_box_props_deser::deserialize_text_box_props, kstyle::kstyle_deser::deserialize_kstyle}, serialized::STextBoxBundle};


pub fn deserialize_text_box_bundle(store: &KayakStore, bb: STextBoxBundle) -> Result<TextBoxBundle, &'static str>  {
    TextBoxBundleBuilder::new(store, bb).build().deserialize()
}

pub struct TextBoxBundleBuilder<'a> {
    store: &'a KayakStore,
    node: STextBoxBundle,
}
impl<'a> TextBoxBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: STextBoxBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn text_box(&self) -> Option<TextBoxProps> {
        let prop = &self.node.text_box.clone();
        if let Some(val) = prop {
            deserialize_text_box_props(val.to_owned()).ok()
        } else {
            None
        }
        
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        if let Some(val) = prop {
            deserialize_kstyle(val.to_owned()).ok()
        } else {
            None
        }        
    }

    fn widget_name(&self) -> String {
        let prop = &self.node.name.clone();
        prop.to_owned()
    }

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.styles.to_owned());
        self
    }

    pub fn deserialize(&self) -> Result<TextBoxBundle, &'static str> {                        
        let text_box = self.text_box();
        let styles = self.styles();
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