use bevy::{prelude::{Handle, Image, AssetServer}, ui::{UiImage}};
use kayak_ui::{widgets::KImage};

use crate::{ui_parser::Conv, kayak::store::KayakStore};

use super::simage::SImage;

pub fn deserialize_image(store: &KayakStore, ib: SImage) -> Result<KImage, &'static str>  {
    ImageDeser::new(store, ib).deserialize_kimage()
}

pub fn deserialize_image_ui(store: &KayakStore, ib: SImage) -> Result<UiImage, &'static str>  {
    ImageDeser::new(store, ib).deserialize_ui_image()
}

pub fn deserialize_image_handle(store: &KayakStore, ib: SImage) -> Result<Handle<Image>, &'static str>  {
    ImageDeser::new(store, ib).handle()
}

pub struct ImageDeser<'a> {
    store: &'a KayakStore,
    node: SImage,
}
impl<'a> ImageDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SImage) -> Self {
        Self {
            store,
            node
        }
    }

    fn image_path(&self) -> Option<String> {
        let prop = &self.node.path.clone();
        Conv::get_prop(prop)
    }    

    fn asset_server(&self) -> AssetServer {
        self.store.asset_server.clone()
    }

    fn image_ref(&self) -> Option<Handle<Image>> {
        let prop = &self.node.ref_id.clone();
        if let Some(img_id) = prop {
            let opt_image = self.store.assets.image(img_id);
            if let Some(img_handle) = opt_image {
                Some(img_handle.to_owned())
            } else {
                None
            }            
        } else {
            None
        }
    }

    fn image(&self) -> Option<Handle<Image>> {
        if let Some(img_path) = self.image_path() {
            let image = self.asset_server().load(img_path);
            Some(image)
        } else {
            if let Some(img) = self.image_ref() {
                Some(img)
            } else {
                None
            }            
        }
    }

    pub fn handle(&self) -> Result<Handle<Image>, &'static str> {            
        let image = self.image().unwrap();         
        Ok(image)
    }

    pub fn deserialize_kimage(&self) -> Result<KImage, &'static str> {            
        let image = self.image().unwrap();         
        let kimage = KImage(image);
        Ok(kimage)
    }

    pub fn deserialize_ui_image(&self) -> Result<UiImage, &'static str> {            
        let image = self.image().unwrap();         
        let ui_image = UiImage(image);
        Ok(ui_image)
    }
}
