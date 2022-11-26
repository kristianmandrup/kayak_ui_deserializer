use kayak_ui::{widgets::{TextWidgetBundle, TextProps}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::{STextWidgetBundle}, kayak_store::KayakStore, ui_text_props::TextPropsBuilder};

pub fn build_text_widget_bundle(store: &KayakStore, tab: STextWidgetBundle) -> Result<TextWidgetBundle, &'static str>  {
    TextWidgetBundleBuilder::new(store, tab).build().parse()
}


pub struct TextWidgetBundleBuilder<'a> {
    store: &'a KayakStore,
    node: STextWidgetBundle,
}
impl<'a> TextWidgetBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: STextWidgetBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn text(&self) -> Option<TextProps> {
        let prop = &self.node.text;
        if let Some(val) = prop.to_owned() {
            TextPropsBuilder::new(val).parse().ok()
        } else {
            None
        }        
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
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

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.styles.to_owned());
        self
    }

    pub fn parse(&self) -> Result<TextWidgetBundle, &'static str> {                        
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