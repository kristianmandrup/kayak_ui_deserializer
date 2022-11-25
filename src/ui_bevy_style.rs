use bevy::ui::{Display, Style, PositionType, Direction, FlexDirection, FlexWrap, AlignItems, AlignSelf, AlignContent, JustifyContent, Size, Val, UiRect, Overflow};

use crate::{json_deserializer::SBevyStyle, ui_rect::{UiRectBuilder, to_val}, ui_parser::Conv, ui_size::SizeBuilder};

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

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn display(&self) -> Option<Display> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "flex" => Some(Display::Flex),
                _ => Some(Display::None)
            }
        } else {
            None
        }
    }

    fn position_type(&self) -> Option<PositionType> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "relative" => Some(PositionType::Relative),
                "absolute" => Some(PositionType::Absolute),
                _ => None
            }
        } else {
            None
        }
    }

    fn direction(&self) -> Option<Direction> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "inherit" => Some(Direction::Inherit),
                "ltr" => Some(Direction::LeftToRight),
                "rtl" => Some(Direction::RightToLeft),
                _ => None
            }
        } else {
            None
        }
    }

    fn flex_direction(&self) -> Option<FlexDirection> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "row" => Some(FlexDirection::Row),
                "col" => Some(FlexDirection::Column),
                "rowreverse" => Some(FlexDirection::RowReverse),
                "colreverse" => Some(FlexDirection::ColumnReverse),
                _ => None
            }
        } else {
            None
        }
    }

    fn flex_wrap(&self) -> Option<FlexWrap> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "nowrap" => Some(FlexWrap::NoWrap),
                "wrap" => Some(FlexWrap::Wrap),
                "wrapreverse" => Some(FlexWrap::WrapReverse),
                _ => None
            }
        } else {
            None
        }
    }
    
    fn align_items(&self) -> Option<AlignItems> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "flexstart" => Some(AlignItems::FlexStart),
                "flexend" => Some(AlignItems::FlexEnd),
                "center" => Some(AlignItems::Center),
                "baseline" => Some(AlignItems::Baseline),
                "stretch" => Some(AlignItems::Stretch),
                _ => None
            }
        } else {
            None
        }
    }

    fn align_self(&self) -> Option<AlignSelf> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "auto" => Some(AlignSelf::Auto),
                "flexstart" => Some(AlignSelf::FlexStart),
                "flexend" => Some(AlignSelf::FlexEnd),
                "center" => Some(AlignSelf::Center),
                "baseline" => Some(AlignSelf::Baseline),
                "stretch" => Some(AlignSelf::Stretch),
                _ => None
            }
        } else {
            None
        }
    }

    fn align_content(&self) -> Option<AlignContent> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "flexstart" => Some(AlignContent::FlexStart),
                "flexend" => Some(AlignContent::FlexEnd),
                "center" => Some(AlignContent::Center),
                "stretch" => Some(AlignContent::Stretch),
                "spacebetween" => Some(AlignContent::SpaceBetween),                
                "spacearound" => Some(AlignContent::SpaceAround),                
                _ => None
            }
        } else {
            None
        }
    }

    fn justify_content(&self) -> Option<JustifyContent> {
        let prop = &self.node.display.clone();
        if let Some(val) = prop.to_owned() {
            match val.as_str() {
                "flexstart" => Some(JustifyContent::FlexStart),
                "flexend" => Some(JustifyContent::FlexEnd),
                "center" => Some(JustifyContent::Center),
                "spacebetween" => Some(JustifyContent::SpaceBetween),                
                "spacearound" => Some(JustifyContent::SpaceAround),                
                "spaceevenly" => Some(JustifyContent::SpaceEvenly),                
                _ => None
            }
        } else {
            None
        }
    }

    fn position(&self) -> Option<UiRect> {
        let prop = &self.node.position.clone();
        if let Some(val) = prop.clone() {
            UiRectBuilder::new(val).parse().ok()
        } else {
            None
        }
        
    }
    
    fn margin(&self) -> Option<UiRect> {
        let prop = &self.node.margin.clone();
        if let Some(val) = prop.clone() {
            UiRectBuilder::new(val).parse().ok()
        } else {
            None
        }

    }

    fn padding(&self) -> Option<UiRect> {
        let prop = &self.node.padding.clone();
        if let Some(val) = prop.clone() {
            UiRectBuilder::new(val).parse().ok()
        } else {
            None
        }
    }

    fn border(&self) -> Option<UiRect> {
        let prop = &self.node.border.clone();
        if let Some(val) = prop.clone() {
            UiRectBuilder::new(val).parse().ok()
        } else {
            None
        }
    }

    pub fn flex_grow(&self) -> Option<f32> {
        let prop = &self.node.flex_grow.clone();
        self.to_f32(prop)
    }

    pub fn flex_shrink(&self) -> Option<f32> {
        let prop = &self.node.flex_shrink.clone();
        self.to_f32(prop)
    }

    pub fn flex_basis(&self) -> Option<Val> {
        let prop = &self.node.flex_basis.clone();
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }        
    }

    pub fn size(&self) -> Option<Size> {
        let prop = &self.node.size.clone();
        if let Some(val) = prop.clone() {
            SizeBuilder::new(val).parse().ok()
        } else {
            None
        }        
    }

    pub fn min_size(&self) -> Option<Size> {
        let prop = &self.node.min_size.clone();
        if let Some(val) = prop.clone() {
            SizeBuilder::new(val).parse().ok()
        } else {
            None
        }        
    }

    pub fn max_size(&self) -> Option<Size> {
        let prop = &self.node.max_size.clone();
        if let Some(val) = prop.clone() {
            SizeBuilder::new(val).parse().ok()
        } else {
            None
        }        
    }

    pub fn aspect_ratio(&self) -> Option<f32> {
        let prop = &self.node.aspect_ratio.clone();
        self.to_f32(prop)
    }

    pub fn overflow(&self) -> Option<Overflow> {
        let prop = &self.node.overflow.clone();
        if let Some(val) = prop.clone() {
            let ov = match val.as_str() {
                "visible" => Overflow::Visible,
                "hidden" => Overflow::Hidden,
                _ => Overflow::Visible
            };
            Some(ov)
        } else {
            None
        }        
    }

    pub fn parse(&self) -> Result<Style, &'static str> {                        
        let display = self.display();
        let position_type = self.position_type();
        let direction = self.direction();
        let flex_direction = self.flex_direction();
        let flex_wrap = self.flex_wrap();
        let align_items = self.align_items();
        let align_self = self.align_self();
        let align_content = self.align_content();
        let justify_content = self.justify_content();
        let position = self.position();
        let margin = self.margin();
        let padding = self.padding();        
        let border = self.border(); 
        let flex_grow = self.flex_grow();
        let flex_shrink = self.flex_shrink();
        let flex_basis = self.flex_basis();
        let size = self.size();
        let min_size = self.min_size();
        let max_size = self.max_size();
        let aspect_ratio = self.aspect_ratio();
        let overflow = self.overflow();
        
        let mut style = Style::default();

        if let Some(val) = display {
            style.display = val;    
        }
        if let Some(val) = position_type {
            style.position_type = val;    
        }
        if let Some(val) = direction {
            style.direction = val;    
        }
        if let Some(val) = flex_direction {
            style.flex_direction = val;    
        }
        if let Some(val) = flex_wrap {
            style.flex_wrap = val;    
        }
        if let Some(val) = align_items {
            style.align_items = val;    
        }
        if let Some(val) = align_self {
            style.align_self = val;    
        }
        if let Some(val) = align_content {
            style.align_content = val;    
        }
        if let Some(val) = justify_content {
            style.justify_content = val;    
        }
        if let Some(val) = position {
            style.position = val;    
        }
        if let Some(val) = margin {
            style.margin = val;    
        }
        if let Some(val) = border {
            style.border = val;    
        }
        if let Some(val) = padding {
            style.padding = val;    
        }
        if let Some(val) = flex_grow {
            style.flex_grow = val;    
        }
        if let Some(val) = flex_shrink {
            style.flex_shrink = val;    
        }
        if let Some(val) = flex_basis {
            style.flex_basis = val;    
        }
        if let Some(val) = size {
            style.size = val;    
        }
        if let Some(val) = min_size {
            style.min_size = val;    
        }
        if let Some(val) = max_size {
            style.max_size = val;    
        }
        style.aspect_ratio = aspect_ratio;
        if let Some(val) = overflow {
            style.overflow = val;    
        }
        Ok(style)       
    }    

}

// /// How to handle overflow
// pub overflow: Overflow,