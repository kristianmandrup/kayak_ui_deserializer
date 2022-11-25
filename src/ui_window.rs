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

use kayak_ui::widgets::{WindowBundle, KWindow};

use crate::{json_deserializer::{SWindow, SWindowBundle}, ui_parser::Conv};


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

    pub fn parse(&self) -> Result<KWindow, &'static str> {        
        let mut window = KWindow::default();
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
        
        // let posx = self.posx();
        // let posy = self.posy();
        // let width = self.width();
        // let height = self.height();
        // let z_index = self.z_index();
        // let mut rect = Rect::default();
        let mut window_bundle = WindowBundle::default();
        // if let Some(val) = posx {
        //     rect.posx = val;    
        // }
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