use bevy::{prelude::{ImageBundle}, ui::{UiImage, Style, FocusPolicy, CalculatedSize}};

use crate::{ ui_bevy_style::BevyStyleBuilder, serialized::{SImageBundle}, kayak_store::KayakStore, ui_image::{build_image_ui}, ui_size::SizeBuilder, ui_calc_size::CalcSizeBuilder};


pub fn build_image_bundle(store: &KayakStore, ib: SImageBundle) -> Result<ImageBundle, &'static str>  {
    ImageBundleBuilder::new(store, ib).build().parse()
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
        let prop = &self.node.image;
        if let Some(val) = prop.to_owned() {
            build_image_ui(&self.store, val).ok()
        } else {
            None
        }        
    }

    fn style(&self) -> Option<Style> {
        let prop = &self.node.style;
        if let Some(val) = prop.to_owned() {
            BevyStyleBuilder::new(val).parse().ok()
        } else {
            None
        }        
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

    fn calculated_size(&self) -> Option<CalculatedSize> {
        let prop = &self.node.calculated_size.clone();
        if let Some(val) = prop.clone() {
            CalcSizeBuilder::new(val).parse().ok()
        } else {
            None
        }        
    }


    pub fn build(&self) -> &Self {
        self.store.extend_style(self.node.style.to_owned());
        self
    }

    pub fn parse(&self) -> Result<ImageBundle, &'static str> {                        
        let image = self.image();
        let style = self.style();
        let focus_policy = self.focus_policy();
        let calculated_size = self.calculated_size();
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
        if let Some(val) = calculated_size {
            image_bundle.calculated_size = val;    
        }
        // image_bundle.widget_name = widget_name;
        Ok(image_bundle)       
    }    
}
