use bevy::ui::{Style, Display, PositionType, Direction, FlexDirection, FlexWrap, AlignItems, AlignSelf, AlignContent, JustifyContent, Val, UiRect, Size, Overflow};
use crate::serialized::{SUiRect, SSize, SBevyStyle};

pub fn val_to_str(val: Val) -> String {
    match val {
        Val::Undefined => "".to_string(),
        Val::Auto => "auto".to_string(),
        Val::Px(px) => format!("{} px", px),
        Val::Percent(pct) => format!("{} %", pct),    
    }
}

pub fn to_sui_rect(prop: UiRect) -> SUiRect {
    SUiRect {
        top: Some(val_to_str(prop.top).to_string()),
        right: Some(val_to_str(prop.right).to_string()),
        bottom: Some(val_to_str(prop.bottom).to_string()),
        left: Some(val_to_str(prop.left).to_string()),
    }
}

pub fn to_ssize(prop: Size) -> SSize {
    SSize {
        width: Some(val_to_str(prop.width).to_string()),
        height: Some(val_to_str(prop.height).to_string()),
    }
}


pub struct BevyStyleSerializer {
    node: Style
}
impl BevyStyleSerializer {
    pub fn new(node: Style) -> Self {
        Self {
            node
        }
    }

    fn display(&self) -> &str {
        let prop = &self.node.display.clone();
        match prop.to_owned() {
            Display::Flex => "display",
            _ => "none",
        }    
    }

    fn position_type(&self) -> &str {
        let prop = &self.node.position_type.clone();
        match prop.to_owned() {
            PositionType::Relative => "relative",
            PositionType::Absolute => "absolute",
        }    
    }

    fn direction(&self) -> &str {
        let prop = &self.node.direction.clone();
        match prop {
            Direction::Inherit => "inherit",
            Direction::LeftToRight => "ltr",
            Direction::RightToLeft => "rtl",
        }    
    }

    fn flex_direction(&self) -> &str {
        let prop = &self.node.flex_direction.clone();
        match prop {
            FlexDirection::Row => "row",
            FlexDirection::Column => "col",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse => "col-reverse",
        }
    }

    fn flex_wrap(&self) -> &str {
        let prop = &self.node.flex_wrap.clone();
        match prop {
            FlexWrap::NoWrap =>  "no-wrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse  => "wrap-reverse",
        }
    }

    fn align_items(&self) -> &str {
        let prop = &self.node.align_items.clone();
        match prop {
            AlignItems::FlexStart => "flex-start",
            AlignItems::FlexEnd => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Stretch => "stretch",
        }
    }

    fn align_self(&self) -> &str {
        let prop = &self.node.align_self.clone();
        match prop {
            AlignSelf::Auto => "auto",
            AlignSelf::FlexStart => "flexstart",
            AlignSelf::FlexEnd => "flexend",
            AlignSelf::Center => "center",
            AlignSelf::Baseline => "baseline",
            AlignSelf::Stretch => "stretch",
        }
    }

    fn align_content(&self) -> &str {
        let prop = &self.node.align_content.clone();
        match prop {
            AlignContent::FlexStart => "flex-start",
            AlignContent::FlexEnd => "flex-end",
            AlignContent::Center => "center",
            AlignContent::Stretch => "stretch",
            AlignContent::SpaceBetween => "space-between",
            AlignContent::SpaceAround => "space-around",
        }
    }

    fn justify_content(&self) -> &str {
        let prop = &self.node.justify_content.clone();
        match prop {                
            JustifyContent::FlexStart => "flex-start",
            JustifyContent::FlexEnd => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        }
    }

    fn position(&self) -> SUiRect {
        let prop = &self.node.position.clone();
        to_sui_rect(prop.to_owned())
    }

    fn margin(&self) -> SUiRect {
        let prop = &self.node.margin.clone();
        to_sui_rect(prop.to_owned())
    }

    fn padding(&self) -> SUiRect {
        let prop = &self.node.padding.clone();
        to_sui_rect(prop.to_owned())
    }

    fn border(&self) -> SUiRect {
        let prop = &self.node.border.clone();
        to_sui_rect(prop.to_owned())
    }

    fn flex_grow(&self) -> String {
        let prop = &self.node.flex_grow.clone();
        prop.to_string()
    }
    
    fn flex_shrink(&self) -> String {
        let prop = &self.node.flex_shrink.clone();
        prop.to_string()
    }

    fn flex_basis(&self) -> String {
        let prop = &self.node.flex_basis.clone();
        val_to_str(prop.to_owned()).to_string()
    }

    fn size(&self) -> SSize {
        let prop = &self.node.size.clone();        
        to_ssize(prop.to_owned())
    }

    fn min_size(&self) -> SSize {
        let prop = &self.node.min_size.clone();        
        to_ssize(prop.to_owned())
    }

    fn max_size(&self) -> SSize {
        let prop = &self.node.max_size.clone();        
        to_ssize(prop.to_owned())
    }

    fn aspect_ratio(&self) -> String {
        let prop = &self.node.aspect_ratio.clone();
        if let Some(val) = prop {
            val.to_string()
        } else {
            "".to_string()
        }        
    }

    fn overflow(&self) -> String {
        let prop = &self.node.overflow.clone();
        match prop.to_owned() {
            Overflow::Visible => "visible".to_string(),
            Overflow::Hidden => "hidden".to_string(),
        }
    }
    
    pub fn serialize(&self) -> Result<SBevyStyle, &'static str> {
        let sstyle = SBevyStyle {
            extends: None,
            display: Some(self.display().to_string()),
            position_type: Some(self.position_type().to_string()),
            direction: Some(self.direction().to_string()),
            flex_direction: Some(self.flex_direction().to_string()),
            flex_wrap: Some(self.flex_wrap().to_string()),
            align_items: Some(self.align_items().to_string()),
            align_self: Some(self.align_self().to_string()),
            align_content: Some(self.align_content().to_string()),
            justify_content: Some(self.justify_content().to_string()),
            position: None,
            position_obj: Some(self.position()),
            margin: None,
            margin_obj: Some(self.margin()),
            padding: None,
            padding_obj: Some(self.padding()),
            border: None,
            border_obj: Some(self.border()),
            flex_grow: Some(self.flex_grow()),
            flex_shrink: Some(self.flex_shrink()),
            flex_basis: Some(self.flex_basis()),
            size: None,
            size_obj: Some(self.size()),
            min_size: None,
            min_size_obj: Some(self.min_size()),
            max_size: None,
            max_size_obj: Some(self.max_size()),
            aspect_ratio: Some(self.aspect_ratio()),
            overflow: Some(self.overflow())
        };
        Ok(sstyle)       
    }    
}