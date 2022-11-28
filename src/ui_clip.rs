use kayak_ui::{widgets::{Clip, ClipBundle}, prelude::{KStyle, WidgetName}};

use crate::{serialized::SClipBundle, kayak_store::KayakStore, kstyle::ui_kstyle::KStyleBuilder};


pub fn build_clip_bundle(store: &KayakStore, cb: SClipBundle) -> Result<ClipBundle, &'static str>  {
    ClipBundleBuilder::new(store, cb).build().parse()
}
pub struct ClipBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SClipBundle,
}
impl<'a> ClipBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SClipBundle) -> Self {
        Self {
            store,            
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

    pub fn parse(&self) -> Result<ClipBundle, &'static str> {                        
        let clip = self.clip();
        let styles = self.styles();
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