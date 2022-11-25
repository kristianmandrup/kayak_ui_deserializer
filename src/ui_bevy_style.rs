use bevy::ui::{Display, Style, PositionType, Direction, FlexDirection, FlexWrap, AlignItems, AlignSelf};

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
                "rowrev" => Some(FlexDirection::RowReverse),
                "colrev" => Some(FlexDirection::ColumnReverse),
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
                "wraprev" => Some(FlexWrap::WrapReverse),
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

    pub fn parse(&self) -> Result<Style, &'static str> {                        
        let display = self.display();
        let position_type = self.position_type();
        let direction = self.direction();
        let flex_direction = self.flex_direction();
        let flex_wrap = self.flex_wrap();
        let align_items = self.align_items();
        let align_self = self.align_self();
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
        Ok(style)       
    }    

}

// /// How to align each line, only applies if flex_wrap is set to
// /// [`FlexWrap::Wrap`] and there are multiple lines of items
// pub align_content: AlignContent,
// /// How items align according to the main axis
// pub justify_content: JustifyContent,
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