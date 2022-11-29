use kayak_ui::{widgets::{TextWidgetBundle, TextProps}, prelude::{KStyle, WidgetName}};

use crate::{kayak::{store::KayakStore, text_props::text_props_deser::deserialize_text_props, kstyle::kstyle_deser::deserialize_kstyle}, serialized::STextWidgetBundle};

pub fn deserialize_text_widget_bundle(store: &KayakStore, tab: STextWidgetBundle) -> Result<TextWidgetBundle, &'static str>  {
    TextWidgetBundleDeser::new(store, tab).build().deserialize()
}

pub struct TextWidgetBundleDeser<'a> {
    store: &'a KayakStore,
    node: STextWidgetBundle,
}
impl<'a> TextWidgetBundleDeser<'a> {
    pub fn new(store: &'a KayakStore, node: STextWidgetBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn text(&self) -> Option<TextProps> {
        let prop = &self.node.text;
        if let Some(val) = prop.to_owned() {
            deserialize_text_props(self.store, val).ok()
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

    pub fn deserialize(&self) -> Result<TextWidgetBundle, &'static str> {                        
        let text = self.text();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut twb = TextWidgetBundle::default();
        if let Some(val) = text {
            twb.text = val;    
        }
        if let Some(val) = styles {
            twb.styles = val;    
        }
        twb.widget_name = WidgetName(name);            
        Ok(twb)       
    }       
}