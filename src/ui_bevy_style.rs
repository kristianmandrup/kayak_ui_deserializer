use crate::json_deserializer::SBevyStyle;

// Style
pub struct BevyStyleBuilder {
    node: SBevyStyle
}
impl BevyStyleBuilder {
    pub fn new(node: SBevyStyle) -> Self {
        Self {
            node
        }
    }
}