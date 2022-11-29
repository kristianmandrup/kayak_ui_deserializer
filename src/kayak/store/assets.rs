use std::collections::HashMap;

use bevy::{prelude::{Handle, Image}, text::Font};

pub struct StoredAssets {
    pub images: HashMap<String, Handle<Image>>,
    pub fonts: HashMap<String, Handle<Font>>
}
impl StoredAssets {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            fonts: HashMap::new(),
        }                    
    }

    pub fn image(&self, id: &str) -> Option<&Handle<Image>> {
        self.images.get(id)
    }    

    pub fn font(&self, id: &str) -> Option<&Handle<Font>> {
        self.fonts.get(id)
    }    
}