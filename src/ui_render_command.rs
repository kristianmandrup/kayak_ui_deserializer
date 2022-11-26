use std::any::Any;
use kayak_ui::prelude::RenderCommand;
use crate::{ui_alignment::to_alignment, serialized::OptStr};

pub struct UiRenderCommandText {
    pub content: OptStr,
    pub alignment: OptStr,
    pub word_wrap: OptStr,
}

pub struct RenderCommandBuilder {
    pub node: Box<dyn Any + 'static>,
    // asset_server: AssetServer
}
impl RenderCommandBuilder {
    // asset_server: AssetServer
    pub fn new(node: Box<dyn Any> ) -> Self {
        Self {
            node,
            // asset_server
        }
    }

    // pub fn parse(&self) -> Result<RenderCommand, &'static str> {        
    //     let command = self.node.render_command.clone();    
    //     Ok(self.to_render_command(command.unwrap()))
    // }

    pub fn to_render_command(&self, command: String) -> RenderCommand {
        match command.to_lowercase().as_str() {
            "empty" => RenderCommand::Empty,
            "layout" => RenderCommand::Layout,
            "clip" => RenderCommand::Clip,
            "quad" => RenderCommand::Quad,
            "text" => {
                let component_ref: &dyn Any = &*self.node;
                let node = component_ref.downcast_ref::<UiRenderCommandText>().unwrap();
                let content = node.content.clone().unwrap();
                let alignment = to_alignment(node.alignment.clone().unwrap());
                let word_wrap = node.word_wrap.clone().unwrap().trim().parse().unwrap();
                RenderCommand::Text {
                    content,
                    alignment,
                    word_wrap
                }
            },
            // "image" => {
            //     let asset_path = self.node.asset_path.clone().unwrap();
            //     let handle = self.asset_server.load(asset_path);
            //     RenderCommand::Image {
            //         handle
            //     }
            // },
            _ => RenderCommand::Empty,
        }        
    }
}

// pub enum RenderCommand {
//     Empty,
//     /// Represents a node that has no renderable object but contributes to the layout.
//     Layout,
//     Clip,
//     Quad,
//     Text {
//         content: String,
//         alignment: Alignment,
//         word_wrap: bool,
//     },
//     Image {
//         handle: Handle<Image>,
//     },
//     TextureAtlas {
//         position: Vec2,
//         size: Vec2,
//         handle: Handle<Image>,
//     },
//     NinePatch {
//         border: Edge<f32>,
//         handle: Handle<Image>,
//     },
// }
