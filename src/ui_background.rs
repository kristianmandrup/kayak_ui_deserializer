use kayak_ui::{widgets::{Background, BackgroundBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::SBackgroundBundle};


pub fn build_background_bundle(bb: SBackgroundBundle) -> Result<BackgroundBundle, &'static str>  {
    BackgroundBundleBuilder::new(bb).parse()
}

pub struct BackgroundBundleBuilder {
    node: SBackgroundBundle,
}
impl BackgroundBundleBuilder {
    pub fn new(node: SBackgroundBundle) -> Self {
        Self {
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
            KStyleBuilder::new(val.to_owned()).parse().ok()
        } else {
            None
        }        
    }

    fn widget_name(&self) -> String {
        let prop = &self.node.name.clone();
        prop.to_owned()
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