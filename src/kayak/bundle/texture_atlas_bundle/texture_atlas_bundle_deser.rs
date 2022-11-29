use kayak_ui::{widgets::{TextureAtlasProps, TextureAtlasBundle}, prelude::{KStyle, WidgetName}};

use crate::{kayak::{store::KayakStore, texture_atlas_props::texture_atlas_props_deser::deserialize_texture_atlas_props, kstyle::kstyle_deser::deserialize_kstyle}};

use super::stexture_atlas_bundle::STextureAtlasBundle;

pub fn build_texture_atlas_bundle(store: &KayakStore, tab: STextureAtlasBundle) -> Result<TextureAtlasBundle, &'static str>  {
    TextureAtlasBundleDeser::new(store, tab).build().parse()
}


pub struct TextureAtlasBundleDeser<'a> {
    store: &'a KayakStore,
    node: STextureAtlasBundle,
}
impl<'a> TextureAtlasBundleDeser<'a> {
    pub fn new(store: &'a KayakStore, node: STextureAtlasBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn atlas(&self) -> Option<TextureAtlasProps> {
        let prop = &self.node.atlas.clone();
        if let Some(val) = prop {
            deserialize_texture_atlas_props(val.to_owned()).ok()
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

    pub fn parse(&self) -> Result<TextureAtlasBundle, &'static str> {                        
        let atlas = self.atlas();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut atlas_bundle = TextureAtlasBundle::default();
        if let Some(val) = atlas {
            atlas_bundle.atlas = val;    
        }
        if let Some(val) = styles {
            atlas_bundle.styles = val;    
        }
        atlas_bundle.widget_name = WidgetName(name);            
        Ok(atlas_bundle)       
    }       
}