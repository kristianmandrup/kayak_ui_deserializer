use bevy::asset::HandleId;
use kayak_ui::widgets::NinePatch;
use crate::{kayak::edge::edge_f32_deser::edge_to_str, serialized::OptStr};

use super::snine_patch::SNinePatch;

pub fn serialize_nine_patch(bundle: NinePatch) -> Result<SNinePatch, &'static str>  {
    NinePatchSerializer::new(bundle).serialize()
}

pub struct NinePatchSerializer {
    node: NinePatch,
}
impl NinePatchSerializer {
    pub fn new(node: NinePatch) -> Self {
        Self {
            node
        }
    }

    // fn image(&self) -> String {
    //     let handle = &self.node.handle.clone();
    //     let h = handle.to_owned().id();
    //     match h  {
    //         HandleId::Id(id, num) => format!("{}", id.to_string()),
    //         HandleId::AssetPathId(pathId) => {
    //             let src = pathId.source_path_id();
    //             format!("{}", src)
    //         }
    //     }
    // }


    // fn asset_path(&self) -> Str {
    //     let prop = &self.node.asset_path.clone();
    //     prop.to_owned()
    // }


    fn border(&self) -> OptStr {
        let prop = self.node.border.to_owned();
        Some(edge_to_str(prop))
    }

    pub fn serialize(&self) -> Result<SNinePatch, &'static str> {
        let border = self.border();
        let mut button = SNinePatch {
            border,
            image: None,
            border_obj: None,
            asset_path: None
        };
        Ok(button)
    }
}