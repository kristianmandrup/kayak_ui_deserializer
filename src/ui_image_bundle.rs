use bevy::{prelude::{ImageBundle}, ui::{UiImage, Style, FocusPolicy}};

use crate::{ ui_bevy_style::BevyStyleBuilder, serialized::{SImageBundle}, kayak_store::KayakStore, ui_image::{build_image_ui}};


pub fn build_image_bundle(store: &KayakStore, ib: SImageBundle) -> Result<ImageBundle, &'static str>  {
    ImageBundleBuilder::new(store, ib).parse()
}

pub struct ImageBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SImageBundle,
}
impl<'a> ImageBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SImageBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn image(&self) -> Option<UiImage> {
        let prop = &self.node.image.clone();
        build_image_ui(&self.store, prop.to_owned()).ok()
    }

    fn style(&self) -> Option<Style> {
        let prop = &self.node.styles.clone();
        BevyStyleBuilder::new(prop.to_owned()).parse().ok()
    }

    fn focus_policy(&self) -> Option<FocusPolicy> {
        let prop = &self.node.focus_policy.clone();
        if let Some(val) = prop {
            let fp = match val.as_str() {
                "block" => FocusPolicy::Block,
                "pass" => FocusPolicy::Pass,
                _ => FocusPolicy::Block
            };
            Some(fp)
        } else {
            None
        }            
    }

    pub fn parse(&self) -> Result<ImageBundle, &'static str> {                        
        let image = self.image();
        let style = self.style();
        let focus_policy = self.focus_policy();
        // let widget_name = self.widget_name();
        // let children = self.children();
        let mut image_bundle = ImageBundle::default();
        if let Some(val) = image {
            image_bundle.image = val;    
        }
        if let Some(val) = style {
            image_bundle.style = val;    
        }
        if let Some(val) = focus_policy {
            image_bundle.focus_policy = val;    
        }
        // image_bundle.widget_name = widget_name;
        Ok(image_bundle)       
    }    
}
