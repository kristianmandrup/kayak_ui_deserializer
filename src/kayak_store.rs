use std::collections::HashMap;

use bevy::{prelude::{ImageBundle, AssetServer}, ui::Style};
use kayak_ui::{prelude::KStyle, widgets::{KButton, TextWidgetBundle, TextBoxBundle, WindowBundle, TextureAtlasBundle, KButtonBundle, BackgroundBundle, ClipBundle, ElementBundle}};

use crate::{store::{StoredWidgets, StoredBundles, StoredAssets}, serialized::{SKStyle, SBevyStyle}};

// #[derive(Copy)]
pub struct KayakStore {
    pub asset_server: AssetServer,
    // pub assets: HashMap<String, Asset>,
    pub kstyles: HashMap<String, KStyle>,
    pub styles: HashMap<String, Style>,
    pub widgets: StoredWidgets,
    pub bundles: StoredBundles,
    pub assets: StoredAssets,
    
}
impl KayakStore {
    pub fn new(asset_server: AssetServer) -> Self {
        Self {
            asset_server,
            kstyles: HashMap::new(),
            styles: HashMap::new(),
            widgets: StoredWidgets::new(),
            bundles: StoredBundles::new(),
            assets: StoredAssets::new()
        }        
    }

    fn kstyle_extend(&self, mut kstyle: KStyle, id: &str) -> KStyle  {
        let extension = self.kstyle(id);
        if let Some(ext) = extension {
            ext.clone_into(&mut kstyle);
            // kstyle.apply(ext);
            kstyle
        } else {
            kstyle
        }
    }

    fn style_extend(&self, mut style: Style, id: &str) -> Style  {
        let extension = self.style(id);
        if let Some(ext) = extension {
            ext.clone_into(&mut style);
            style
        } else {
            style
        }
    }

    pub fn extend_style(&self, styles: Option<SBevyStyle>) {
        if let Some(styles) = styles {
            let extends = styles.extends;
            if let Some(id) = extends {
                let style = self.style(id.as_str());
                if let Some(stl) = style {
                    self.style_extend(stl.to_owned(), id.as_str());
                }                        
            }
        }
    }

    pub fn extend_kstyle(&self, kstyles: Option<SKStyle>) {
        if let Some(styles) = kstyles {
            let extends = styles.extends;
            if let Some(id) = extends {
                let style = self.kstyle(id.as_str());
                if let Some(stl) = style {
                    self.kstyle_extend(stl.to_owned(), id.as_str());
                }                        
            }
        }                
    }
    

    pub fn kstyle(&self, id: &str) -> Option<&KStyle> {
        self.kstyles.get(id)
    }

    pub fn style(&self, id: &str) -> Option<&Style> {
        self.styles.get(id)
    }

    pub fn button(&self, id: &str) -> Option<&KButton> {
        self.widgets.button(id)
    }

    pub fn text_widget_bundle(&self, id: &str) -> Option<&TextWidgetBundle> {
        self.bundles.text_widget_bundle(id)
    }

    pub fn text_box_bundle(&self, id: &str) -> Option<&TextBoxBundle> {
        self.bundles.text_box_bundle(id)
    }

    pub fn window_bundle(&self, id: &str) -> Option<&WindowBundle> {
        self.bundles.window_bundle(id)
    }

    pub fn texture_atlas_bundle(&self, id: &str) -> Option<&TextureAtlasBundle> {
        self.bundles.texture_atlas_bundle(id)
    }

    pub fn image_bundle(&self, id: &str) -> Option<&ImageBundle> {
        self.bundles.image_bundle(id)
    }

    pub fn button_bundle(&self, id: &str) -> Option<&KButtonBundle> {
        self.bundles.button_bundle(id)
    }

    pub fn background_bundle(&self, id: &str) -> Option<&BackgroundBundle> {
        self.bundles.background_bundle(id)
    }

    pub fn clip_bundle(&self, id: &str) -> Option<&ClipBundle> {
        self.bundles.clip_bundle(id)
    }

    pub fn element_bundle(&self, id: &str) -> Option<&ElementBundle> {
        self.bundles.element_bundle(id)
    }
}
