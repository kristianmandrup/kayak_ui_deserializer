use kayak_ui::{widgets::{Background, BackgroundBundle}, prelude::{KStyle, WidgetName}};

use crate::{serialized::SBackgroundBundle, kayak::{store::KayakStore, kstyle::kstyle_deser::{deserialize_kstyle}}};


pub fn build_background_bundle(store: &KayakStore, bb: SBackgroundBundle) -> Result<BackgroundBundle, &'static str>  {
    BackgroundBundleDeser::new(store, bb).build().parse()
}

pub struct BackgroundBundleDeser<'a> {
    store: &'a KayakStore,
    node: SBackgroundBundle,
}
impl<'a> BackgroundBundleDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SBackgroundBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn background(&self) -> Option<Background> {
        let prop = &self.node.background.clone();
        if let Some(_) = prop {
            Some(Background {})
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

    pub fn parse(&self) -> Result<BackgroundBundle, &'static str> {                        
        let background = self.background();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut background_bundle = BackgroundBundle::default();
        if let Some(val) = background {
            background_bundle.background = val;    
        }
        if let Some(val) = styles {
            background_bundle.styles = val;    
        }
        background_bundle.widget_name = WidgetName(name);            
        Ok(background_bundle)       
    }    
}