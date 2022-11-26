use kayak_ui::{widgets::{Clip, ClipBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::SClipBundle};


pub fn build_clip_bundle(cb: SClipBundle) -> Result<ClipBundle, &'static str>  {
    ClipBundleBuilder::new(cb).parse()
}

pub struct ClipBundleBuilder {
    node: SClipBundle,
}
impl ClipBundleBuilder {
    pub fn new(node: SClipBundle) -> Self {
        Self {
            node
        }
    }

    fn clip(&self) -> Option<Clip> {
        let prop = &self.node.clip.clone();
        if let Some(_) = prop {
            Some(Clip {})
        } else {
            None
        }        
    }

    fn style(&self) -> Option<KStyle> {
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

    pub fn parse(&self) -> Result<ClipBundle, &'static str> {                        
        let clip = self.clip();
        let styles = self.style();
        let name = self.widget_name();
        // let children = self.children();
        let mut clip_bundle = ClipBundle::default();
        if let Some(val) = clip {
            clip_bundle.clip = val;    
        }
        if let Some(val) = styles {
            clip_bundle.styles = val;    
        }
        clip_bundle.widget_name = WidgetName(name);            
        Ok(clip_bundle)       
    }    
}