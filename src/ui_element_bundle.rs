use kayak_ui::{widgets::{Element, ElementBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::SElementBundle};


pub fn build_element_bundle(bb: SElementBundle) -> Result<ElementBundle, &'static str>  {
    ElementBundleBuilder::new(bb).parse()
}

pub struct ElementBundleBuilder {
    node: SElementBundle,
}
impl ElementBundleBuilder {
    pub fn new(node: SElementBundle) -> Self {
        Self {
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