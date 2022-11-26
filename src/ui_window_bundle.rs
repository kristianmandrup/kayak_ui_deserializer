use kayak_ui::{widgets::{WindowBundle, KWindow}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::{SWindowBundle}, ui_window::WindowBuilder, kayak_store::KayakStore};

pub fn build_window_bundle(store: &KayakStore, wb: SWindowBundle) -> Result<WindowBundle, &'static str>  {
    WindowBundleBuilder::new(store, wb).build().parse()
}

pub struct WindowBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SWindowBundle,
}
impl<'a> WindowBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SWindowBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn window(&self) -> Option<KWindow> {
        let prop = &self.node.window.clone();
        if let Some(val) = prop {
            WindowBuilder::new(self.store, val.to_owned()).parse().ok()
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

    pub fn parse(&self) -> Result<WindowBundle, &'static str> {                        
        let window = self.window();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut window_bundle = WindowBundle::default();
        if let Some(val) = window {
            window_bundle.window = val;    
        }
        if let Some(val) = styles {
            window_bundle.styles = val;    
        }
        window_bundle.widget_name = WidgetName(name);            
        Ok(window_bundle)       
    }    
}