// pub struct KWindow {
//     /// If true, allows the window to be draggable by its title bar
//     pub draggable: bool,
//     /// The initial position at which to display the window in pixels
//     pub initial_position: Vec2,
//     /// The size of the window in pixels
//     pub size: Vec2,
//     /// The text to display in the window's title bar
//     pub title: String,
//     /// Styles for the main window quad.
//     pub window_styles: KStyle,
//     /// A set of styles to apply to the children element wrapper.
//     pub children_styles: KStyle,
// }

// pub struct WindowBundle {
//     pub window: KWindow,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub widget_name: WidgetName,
// }

use bevy::prelude::Vec2;
use kayak_ui::{widgets::{WindowBundle, KWindow}, prelude::KStyle};

use crate::{json_deserializer::{SWindow, SWindowBundle}, ui_parser::Conv, ui_style::StyleBuilder};


pub struct WindowBuilder {
    node: SWindow,
}
impl WindowBuilder {
    pub fn new(node: SWindow) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn draggable(&self) -> Option<bool> {
        let prop = &self.node.draggable.clone();
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_bool() 
        } else {
            None
        }
    }

    fn to_vec2(&self, prop: &Option<Vec<Option<String>>>) -> Option<Vec2> {
        if let Some(vec) = prop.to_owned() {
            let optx = self.to_f32(&vec[0]);
            let opty = self.to_f32(&vec[1]);
            let xy = (optx, opty);
            if let (Some(x), Some(y)) = xy {
                Some(Vec2::new(x, y))
            } else {
                None
            }            
        } else {
            None
        }
    }

    fn initial_position(&self) -> Option<Vec2> {
        let prop = &self.node.initial_position.clone();
        self.to_vec2(prop)
    }

    fn size(&self) -> Option<Vec2> {
        let prop = &self.node.size.clone();
        self.to_vec2(prop)
    }

    fn title(&self) -> Option<String> {
        let prop = &self.node.title.clone();
        prop.to_owned()
    }

    fn window_styles(&self) -> Option<KStyle> {
        let prop = &self.node.window_styles.clone();
        StyleBuilder::new(prop.to_owned()).parse().ok()
    }
    
    fn children_styles(&self) -> Option<KStyle> {
        let prop = &self.node.children_styles.clone();
        StyleBuilder::new(prop.to_owned()).parse().ok()
    }

    pub fn parse(&self) -> Result<KWindow, &'static str> {        
        let draggable = self.draggable();
        let initial_position = self.initial_position();
        let size = self.size();
        let window_styles = self.window_styles();
        let children_styles = self.children_styles();
        let mut window = KWindow::default();
        if let Some(val) = draggable {
            window.draggable = val;    
        }
        if let Some(val) = initial_position {
            window.initial_position = val;    
        }
        if let Some(val) = size {
            window.size = val;    
        }
        if let Some(val) = window_styles {
            window.window_styles = val;    
        }
        if let Some(val) = children_styles {
            window.children_styles = val;    
        }
        Ok(window)
    }
}

pub struct WindowBundleBuilder {
    node: SWindowBundle,
}
impl WindowBundleBuilder {
    pub fn new(node: SWindowBundle) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    pub fn parse(&self) -> Result<WindowBundle, &'static str> {                
        
        // let posy = self.posy();
        // let width = self.width();
        // let height = self.height();
        // let z_index = self.z_index();
        // let mut rect = Rect::default();
        let mut window_bundle = WindowBundle::default();
        // if let Some(val) = posy {
        //     rect.posy = val;    
        // }
        // if let Some(val) = width {
        //     rect.width = val;    
        // }
        // if let Some(val) = height {
        //     rect.height = val;    
        // }
        // if let Some(val) = z_index {
        //     rect.z_index = val;    
        // }
        // Ok(rect)     
        Ok(window_bundle)       
    }    
}