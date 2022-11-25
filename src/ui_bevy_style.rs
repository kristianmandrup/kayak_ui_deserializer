use bevy::ui::{Display, Style, PositionType, Direction, FlexDirection, FlexWrap, AlignItems, AlignSelf, AlignContent, JustifyContent, UiRect};

use crate::{json_deserializer::SBevyStyle, ui_rect::{UiRectBuilder}};

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
        UiRectBuilder::new(prop.clone()).parse().ok()
    }
    
    fn margin(&self) -> Option<UiRect> {
        let prop = &self.node.margin.clone();
        UiRectBuilder::new(prop.clone()).parse().ok()
    }

    fn padding(&self) -> Option<UiRect> {
        let prop = &self.node.padding.clone();
        UiRectBuilder::new(prop.clone()).parse().ok()
    }

    fn border(&self) -> Option<UiRect> {
        let prop = &self.node.border.clone();
        UiRectBuilder::new(prop.clone()).parse().ok()
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
        Ok(style)       
    }    

}

// /// The position of the node as described by its Rect
// pub position: UiRect,
// /// The margin of the node
// pub margin: UiRect,
// /// The padding of the node
// pub padding: UiRect,
// /// The border of the node
// pub border: UiRect,
// /// Defines how much a flexbox item should grow if there's space available
// pub flex_grow: f32,
// /// How to shrink if there's not enough space available
// pub flex_shrink: f32,
// /// The initial size of the item
// pub flex_basis: Val,
// /// The size of the flexbox
// pub size: Size,
// /// The minimum size of the flexbox
// pub min_size: Size,
// /// The maximum size of the flexbox
// pub max_size: Size,
// /// The aspect ratio of the flexbox
// pub aspect_ratio: Option<f32>,
// /// How to handle overflow
// pub overflow: Overflow,