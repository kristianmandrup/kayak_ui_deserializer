use bevy::ui::{Style, Display, PositionType, Direction, FlexDirection, FlexWrap, AlignItems};

pub struct BevyStyleSerializer {
    node: Style
}
impl BevyStyleSerializer {
    pub fn new(node: Style) -> Self {
        Self {
            node
        }
    }

    fn display(&self) -> String {
        let prop = &self.node.display.clone();
        match prop.to_owned() {
            Display::Flex => "display".to_string(),
            _ => "none".to_string(),
        }    
    }

    fn position_type(&self) -> String {
        let prop = &self.node.position_type.clone();
        match prop.to_owned() {
            PositionType::Relative => "relative".to_string(),
            PositionType::Absolute => "absolute".to_string(),
        }    
    }

    fn direction(&self) -> String {
        let prop = &self.node.direction.clone();
        match prop {
            Direction::Inherit => "inherit".to_string(),
            Direction::LeftToRight => "ltr".to_string(),
            Direction::RightToLeft => "rtl".to_string(),
        }    
    }

    fn flex_direction(&self) -> String {
        let prop = &self.node.flex_direction.clone();
        match prop {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::Column => "col".to_string(),
            FlexDirection::RowReverse =>  "rowreverse".to_string(),
            FlexDirection::RowReverse => "row-reverse".to_string(),
            FlexDirection::ColumnReverse =>  "colreverse".to_string(),
            FlexDirection::ColumnReverse => "col-reverse".to_string(),
        }
    }

    fn flex_wrap(&self) -> String {
        let prop = &self.node.flex_wrap.clone();
        match prop {
            FlexWrap::NoWrap =>  "no-wrap".to_string(),
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse  => "wrapreverse".to_string(),
            FlexWrap::WrapReverse  => "wrap-reverse".to_string(),
        }
    }

    fn align_items(&self) -> String {
        let prop = &self.node.align_items.clone();
        match prop {
            AlignItems::FlexStart => "flex-start".to_string(),
            AlignItems::FlexEnd => "flex-end".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
        }
    }

    // fn align_self(&self) -> Option<AlignSelf> {
    //     let prop = &self.node.align_self.clone();
    //     if let Some(val) = prop.to_owned() {
    //         match val.as_str() {
    //             "auto" => Some(AlignSelf::Auto),
    //             "flexstart" => Some(AlignSelf::FlexStart),
    //             "flexend" => Some(AlignSelf::FlexEnd),
    //             "center" => Some(AlignSelf::Center),
    //             "baseline" => Some(AlignSelf::Baseline),
    //             "stretch" => Some(AlignSelf::Stretch),
    //             _ => None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // fn align_content(&self) -> Option<AlignContent> {
    //     let prop = &self.node.align_content.clone();
    //     if let Some(val) = prop.to_owned() {
    //         match val.as_str() {
    //             "flexstart" => Some(AlignContent::FlexStart),
    //             "flex-start" => Some(AlignContent::FlexStart),
    //             "flexend" => Some(AlignContent::FlexEnd),
    //             "flex-end" => Some(AlignContent::FlexEnd),
    //             "center" => Some(AlignContent::Center),
    //             "stretch" => Some(AlignContent::Stretch),
    //             "spacebetween" => Some(AlignContent::SpaceBetween),
    //             "space-between" => Some(AlignContent::SpaceBetween),
    //             "spacearound" => Some(AlignContent::SpaceAround),
    //             "space-around" => Some(AlignContent::SpaceAround),
    //             _ => None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // fn justify_content(&self) -> Option<JustifyContent> {
    //     let prop = &self.node.justify_content.clone();
    //     if let Some(val) = prop.to_owned() {
    //         match val.as_str() {
    //             "flexstart" => Some(JustifyContent::FlexStart),
    //             "flex-start" => Some(JustifyContent::FlexStart),
    //             "flexend" => Some(JustifyContent::FlexEnd),
    //             "flex-end" => Some(JustifyContent::FlexEnd),
    //             "center" => Some(JustifyContent::Center),
    //             "spacebetween" => Some(JustifyContent::SpaceBetween),
    //             "space-between" => Some(JustifyContent::SpaceBetween),
    //             "spacearound" => Some(JustifyContent::SpaceAround),
    //             "space-around" => Some(JustifyContent::SpaceAround),
    //             "spaceevenly" => Some(JustifyContent::SpaceEvenly),
    //             "space-evenly" => Some(JustifyContent::SpaceEvenly),                
    //             _ => None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // pub fn serialize(&self) -> Result<SBevyStyle, &'static str> {
    //     let sstyle = SBevyStyle {}
    //     Ok(sstyle)       
    // }    
}