use bevy::prelude::Vec2;
use kayak_ui::{widgets::{KWindow}, prelude::{KStyle}};

use crate::{kayak::{store::KayakStore, kstyle::kstyle_deser::deserialize_kstyle}, serialized::SWindow, ui_parser::Conv};

pub fn deserialize_window(store: &KayakStore, ib: SWindow) -> Result<KWindow, &'static str>  {
    WindowDeser::new(store, ib).build().deserialize()
}

pub struct WindowDeser<'a> {
    store: &'a KayakStore,
    node: SWindow,
}
impl<'a> WindowDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SWindow) -> Self {
        Self {
            store,
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
        let prop = &self.node.window_styles;
        if let Some(val) = prop.to_owned() {
            deserialize_kstyle(val).ok()
        } else {
            None
        }                        
    }
    
    fn children_styles(&self) -> Option<KStyle> {
        let prop = &self.node.children_styles.clone();
        if let Some(val) = prop.to_owned() {
            deserialize_kstyle(val).ok()
        } else {
            None
        }        
    }

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.window_styles.to_owned());
        self.store.extend_kstyle(self.node.children_styles.to_owned());
        self
    }

    pub fn deserialize(&self) -> Result<KWindow, &'static str> {        
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
