use std::collections::HashMap;

use bevy::prelude::ImageBundle;
use kayak_ui::{prelude::KStyle, widgets::{KButton, TextWidgetBundle, TextBoxBundle, WindowBundle, TextureAtlasBundle, KButtonBundle, BackgroundBundle, ClipBundle, ElementBundle}};

use crate::store::{StoredWidgets, StoredBundles, StoredAssets};

pub struct KayakStore {
    // pub assets: HashMap<String, Asset>,
    pub styles: HashMap<String, KStyle>,
    pub widgets: StoredWidgets,
    pub bundles: StoredBundles,
    pub assets: StoredAssets,
    
}
impl KayakStore {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            widgets: StoredWidgets::new(),
            bundles: StoredBundles::new(),
            assets: StoredAssets::new()
        }        
    }

    pub fn style(&self, id: &str) -> Option<&KStyle> {
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

impl Default for KayakStore {
    fn default() -> Self {
        Self::new()
    }
}
