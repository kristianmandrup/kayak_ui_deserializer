use kayak_ui::{widgets::{TextureAtlasProps, TextureAtlasBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::{STextureAtlasBundle}, kayak_store::KayakStore, ui_texture_atlas::TextureAtlasPropsBuilder};

pub fn build_texture_atlas_bundle(store: &KayakStore, tab: STextureAtlasBundle) -> Result<TextureAtlasBundle, &'static str>  {
    TextureAtlasBundleBuilder::new(store, tab).build().parse()
}


pub struct TextureAtlasBundleBuilder<'a> {
    store: &'a KayakStore,
    node: STextureAtlasBundle,
}
impl<'a> TextureAtlasBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: STextureAtlasBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn atlas(&self) -> Option<TextureAtlasProps> {
        let prop = &self.node.atlas.clone();
        if let Some(val) = prop {
            TextureAtlasPropsBuilder::new(val.to_owned()).parse().ok()
        } else {
            None
        }
        
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        if let Some(val) = prop {
            KStyleBuilder::new(val.to_owned()).parse().ok()
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