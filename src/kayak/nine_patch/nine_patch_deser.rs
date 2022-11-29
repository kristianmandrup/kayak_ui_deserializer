// pub struct NinePatch {
//     /// The handle to nine_patch
//     pub handle: Handle<NinePatch>,
//     /// The size of each edge (in pixels)
//     pub border: Edge<f32>,
// }


// pub struct NinePatchBundle {
//     pub nine_patch: NinePatch,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }

use bevy::prelude::{ Handle, Image};
use kayak_ui::{widgets::NinePatch, prelude::Edge};

use crate::{serialized::SNinePatch, kayak::{store::KayakStore, edge::edge_deser::EdgeDeser}, bevy::image::image_deser::{deserialize_image_handle}};

pub fn deserialize_nine_patch(store: &KayakStore, ib: SNinePatch) -> Result<NinePatch, &'static str>  {
    NinePatchDeser::new(store, ib).deserialize()
}

pub struct NinePatchDeser<'a> {
    store: &'a KayakStore,
    node: SNinePatch,
}
impl<'a> NinePatchDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SNinePatch) -> Self {
        Self {
            store,
            node
        }
    }

    fn handle(&self) -> Option<Handle<Image>> {
        let prop = &self.node.image.clone();
        if let Some(img) = prop.to_owned() {
            deserialize_image_handle(self.store, img).ok()
        } else {
            None
        }        
    }    

    fn border(&self) -> Option<Edge<f32>> {
        if let Some(val) = self.node.border.clone() {
            let edge = val.clone();
            EdgeDeser::create_from_str(edge).deserialize().ok()    
        } else {
            None
        }
    }

    pub fn deserialize(&self) -> Result<NinePatch, &'static str> {            
        let handle = self.handle();         
        let border = self.border();         
        let mut nine_patch = NinePatch::default();
        if let Some(val) = handle {
            nine_patch.handle = val.into();    
        }
        if let Some(val) = border {
            nine_patch.border = val.into();    
        }
        Ok(nine_patch)
    }
}