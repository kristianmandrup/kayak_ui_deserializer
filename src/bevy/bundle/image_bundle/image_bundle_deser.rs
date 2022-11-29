use bevy::{prelude::{ImageBundle, Visibility, ComputedVisibility, Transform}, ui::{UiImage, Style, FocusPolicy, CalculatedSize, BackgroundColor, widget::ImageMode}};

use crate::{kayak::{store::KayakStore, kstyle::kstyle_deser::str_to_color}, serialized::SImageBundle, bevy::{image::image_deser::deserialize_image_ui, style::style_deser::deserialize_style, transform::deserialize_transform, calculated_size::deserialize_calc_size}};


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
            deserialize_image_ui(&self.store, val).ok()
        } else {
            None
        }        
    }

    fn style(&self) -> Option<Style> {
        let prop = &self.node.style;
        if let Some(val) = prop.to_owned() {
            deserialize_style(val).ok()
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

    fn transform(&self) -> Option<Transform> {
        let prop = &self.node.transform.clone();
        if let Some(val) = prop.clone() {
            deserialize_transform(val).ok()
        } else {
            None
        }        
    }

    fn calculated_size(&self) -> Option<CalculatedSize> {
        let prop = &self.node.calculated_size.clone();
        if let Some(val) = prop.clone() {
            deserialize_calc_size(val).ok()
        } else {
            None
        }        
    }

    fn background_color(&self) -> Option<BackgroundColor> {
        let prop = &self.node.background_color.clone();
        if let Some(color) = str_to_color(prop) {
            Some(BackgroundColor(color))
        } else {
            None
        }
    }

    fn image_mode(&self) -> Option<ImageMode> {
        let prop = &&self.node.image_mode.clone();
        if let Some(_) = prop.to_owned() {
            Some(ImageMode::KeepAspect)
        } else {
            None
        }
    }

    fn visibility(&self) -> Option<Visibility> {
        let prop = &&self.node.visibility.clone();
        if let Some(val) = prop.to_owned() {
            if let Some(visible) = val.trim().parse::<bool>().ok() {
                let vis = Visibility {
                    is_visible: visible
                };
                Some(vis)
            } else {
                None
            }            
        } else {
            None
        }
    }

    
    fn computed_visibility(&self) -> Option<ComputedVisibility> {
        let prop = &&self.node.visibility.clone();
        if let Some(val) = prop.to_owned() {
            if let Some(visible) = val.trim().parse::<bool>().ok() {
                let mut vis = ComputedVisibility::default();
                if visible {
                    vis.set_visible_in_view();
                }                
                Some(vis)
            } else {
                None
            }            
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
        let background_color = self.background_color();
        let visibility = self.visibility();
        let computed_visibility = self.computed_visibility();
        let image_mode = self.image_mode();
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
        if let Some(val) = background_color {
            image_bundle.background_color = val;    
        }        
        if let Some(val) = visibility {
            image_bundle.visibility = val;    
        }        
        if let Some(val) = image_mode {
            image_bundle.image_mode = val;    
        }        
        if let Some(val) = computed_visibility {
            image_bundle.computed_visibility = val;    
        }        
        Ok(image_bundle)       
    }    
}
