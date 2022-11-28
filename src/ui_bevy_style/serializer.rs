use bevy::ui::{Style, Display, PositionType, Direction, FlexDirection, FlexWrap, AlignItems, AlignSelf, AlignContent, JustifyContent};

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
            FlexDirection::RowReverse =>  "rowreverse",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse =>  "colreverse",
            FlexDirection::ColumnReverse => "col-reverse",
        }
    }

    fn flex_wrap(&self) -> &str {
        let prop = &self.node.flex_wrap.clone();
        match prop {
            FlexWrap::NoWrap =>  "no-wrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse  => "wrapreverse",
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

    // pub fn serialize(&self) -> Result<SBevyStyle, &'static str> {
    //     let sstyle = SBevyStyle {}
    //     Ok(sstyle)       
    // }    
}