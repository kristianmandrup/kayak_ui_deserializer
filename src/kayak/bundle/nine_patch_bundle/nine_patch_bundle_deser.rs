use kayak_ui::{widgets::{NinePatch, NinePatchBundle}, prelude::{KStyle, WidgetName}};

use crate::{kayak::{store::KayakStore, nine_patch::nine_patch_deser::deserialize_nine_patch, kstyle::kstyle_deser::deserialize_kstyle}};

use super::snine_patch_bundle::SNinePatchBundle;


pub fn build_nine_patch_bundle(store: &KayakStore, bb: SNinePatchBundle) -> Result<NinePatchBundle, &'static str>  {
    NinePatchBundleDeser::new(store, bb).build().deserialize()
}

pub struct NinePatchBundleDeser<'a> {
    store: &'a KayakStore,
    node: SNinePatchBundle,
}
impl<'a> NinePatchBundleDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SNinePatchBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn nine_patch(&self) -> Option<NinePatch> {
        let prop = &self.node.nine_patch.clone();
        if let Some(val) = prop.to_owned() {
            deserialize_nine_patch(self.store, val).ok()
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

    pub fn deserialize(&self) -> Result<NinePatchBundle, &'static str> {                        
        let nine_patch = self.nine_patch();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut np_bundle = NinePatchBundle::default();
        if let Some(val) = nine_patch {
            np_bundle.nine_patch = val;    
        }
        if let Some(val) = styles {
            np_bundle.styles = val;    
        }
        np_bundle.widget_name = WidgetName(name);            
        Ok(np_bundle)       
    }    
}