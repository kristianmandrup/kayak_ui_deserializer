use kayak_ui::prelude::KStyle;

use super::skstyle::SKStyle;

pub struct KStyleSerializer {
    node: KStyle
}
impl KStyleSerializer {
    pub fn new(node: KStyle) -> Self {
        Self {
            node
        }
    }

    pub fn serialize(&self) -> Result<SKStyle, &'static str> {
        Ok(SKStyle::default())
    }    
}