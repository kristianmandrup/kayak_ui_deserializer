use std::collections::HashMap;

use bevy::prelude::ImageBundle;
use kayak_ui::widgets::{TextWidgetBundle, WindowBundle, TextureAtlasBundle, KButtonBundle, BackgroundBundle, ClipBundle, TextBoxBundle, ElementBundle, NinePatchBundle};

pub struct StoredBundles {
    pub text_widget_bundles: HashMap<String, TextWidgetBundle>,
    pub window_bundles: HashMap<String, WindowBundle>,
    pub texture_atlas_bundles: HashMap<String, TextureAtlasBundle>,
    pub image_bundles: HashMap<String, ImageBundle>,
    pub button_bundles: HashMap<String, KButtonBundle>,
    pub background_bundles: HashMap<String, BackgroundBundle>,
    pub clip_bundles: HashMap<String, ClipBundle>,
    pub text_box_bundles: HashMap<String, TextBoxBundle>,
    pub element_bundles: HashMap<String, ElementBundle>,
    pub nine_patch_bundles: HashMap<String, NinePatchBundle>,
}
impl StoredBundles {
    pub fn new() -> Self {
        Self {
            text_widget_bundles: HashMap::new(),
            text_box_bundles: HashMap::new(),            
            window_bundles: HashMap::new(),
            texture_atlas_bundles: HashMap::new(),
            image_bundles: HashMap::new(),
            button_bundles: HashMap::new(),
            background_bundles: HashMap::new(),
            clip_bundles: HashMap::new(),
            element_bundles: HashMap::new(),  
            nine_patch_bundles: HashMap::new(),  
        }                    
    }

    pub fn text_widget_bundle(&self, id: &str) -> Option<&TextWidgetBundle> {
        self.text_widget_bundles.get(id)
    }

    pub fn text_box_bundle(&self, id: &str) -> Option<&TextBoxBundle> {
        self.text_box_bundles.get(id)
    }

    pub fn window_bundle(&self, id: &str) -> Option<&WindowBundle> {
        self.window_bundles.get(id)
    }

    pub fn texture_atlas_bundle(&self, id: &str) -> Option<&TextureAtlasBundle> {
        self.texture_atlas_bundles.get(id)
    }    

    pub fn image_bundle(&self, id: &str) -> Option<&ImageBundle> {
        self.image_bundles.get(id)
    }    

    pub fn button_bundle(&self, id: &str) -> Option<&KButtonBundle> {
        self.button_bundles.get(id)
    }    

    pub fn background_bundle(&self, id: &str) -> Option<&BackgroundBundle> {
        self.background_bundles.get(id)
    }    

    pub fn clip_bundle(&self, id: &str) -> Option<&ClipBundle> {
        self.clip_bundles.get(id)
    }    

    pub fn element_bundle(&self, id: &str) -> Option<&ElementBundle> {
        self.element_bundles.get(id)
    }    

    pub fn nine_patch_bundle(&self, id: &str) -> Option<&NinePatchBundle> {
        self.nine_patch_bundles.get(id)
    }    
}