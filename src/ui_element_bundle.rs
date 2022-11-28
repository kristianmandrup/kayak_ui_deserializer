use kayak_ui::{widgets::{Element, ElementBundle}, prelude::{KStyle, WidgetName}};

use crate::{serialized::SElementBundle, kayak_store::KayakStore, kstyle::ui_kstyle::KStyleBuilder};


pub fn build_element_bundle(store: &KayakStore, bb: SElementBundle) -> Result<ElementBundle, &'static str>  {
    ElementBundleBuilder::new(store, bb).build().parse()
}

pub struct ElementBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SElementBundle,
}
impl<'a> ElementBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SElementBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn element(&self) -> Option<Element> {
        let prop = &self.node.element.clone();
        if let Some(_) = prop {
            Some(Element {})
        } else {
            None
        }
        
    }

    fn style(&self) -> Option<KStyle> {
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

    pub fn parse(&self) -> Result<ElementBundle, &'static str> {                        
        let element = self.element();
        let styles = self.style();
        let name = self.widget_name();
        // let children = self.children();
        let mut element_bundle = ElementBundle::default();
        if let Some(val) = element {
            element_bundle.element = val;    
        }
        if let Some(val) = styles {
            element_bundle.styles = val;    
        }
        element_bundle.widget_name = WidgetName(name);            
        Ok(element_bundle)       
    }    
}