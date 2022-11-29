use bevy::prelude::Color;
use kayak_ui::prelude::{KPositionType, Edge, KStyle, Corner, KCursorIcon, Units, LayoutType};

use crate::{ui_parser::{Conv}, bevy::{color::color_deser::parse_color, corner::corner_deser::{CornerDeser, deserialize_corner}, cursor_icon::to_cursor_icon}, kayak::{edge::{sedge::SEdge, edge_f32_deser::{EdgeDeserF32}, edge_units_deser::to_edge_units}, layout_type::to_layout_type, units::UiUnit}};

use super::skstyle::SKStyle;

pub fn str_to_color(prop: &Option<String>) -> Option<Color> {
    let str = prop.to_owned();
    if let Some(val) = str {
        parse_color(val.as_str())    
    } else {
        None
    }            
}

pub fn obj_to_edge_units(optobj: &Option<SEdge>) -> Option<Edge<Units>> {
    if let Some(obj) = optobj.clone() {
        let top= UiUnit::new(obj.top.to_owned()).parse();
        let left= UiUnit::new(obj.left.to_owned()).parse();
        let right= UiUnit::new(obj.right.to_owned()).parse();
        let bottom= UiUnit::new(obj.bottom.to_owned()).parse();
        let mut edge = Edge::default();
        if let Some(val) = top {
            edge.top = val
        }            
        if let Some(val) = right {
            edge.right = val
        }            
        if let Some(val) = left {
            edge.left = val
        }            
        if let Some(val) = bottom {
            edge.bottom = val
        }            
        Some(edge)            
    } else {
        None
    }    
}

pub fn deserialize_kstyle(sks: SKStyle) -> Result<KStyle, &'static str>  {
    KStyleDeser::new(sks).deserialize()
}


pub struct KStyleDeser {
    node: SKStyle
}
impl KStyleDeser {
    pub fn new(node: SKStyle) -> Self {
        Self {
            node
        }
    }

    fn background_color(&self) -> Option<Color> {
        let prop = &self.node.background_color.clone();
        str_to_color(prop)
    }

    fn border(&self) -> Option<Edge<f32>> {
        if let Some(val) = self.node.border.clone() {
            let edge = val.clone();
            EdgeDeserF32::create_from_str(edge).deserialize().ok()    
        } else {
            None
        }
    }

    fn border_color(&self) -> Option<Color> {
        let prop = &self.node.border_color.clone();
        str_to_color(prop)
    }

    fn border_radius(&self) -> Option<Corner<f32>> {
        if let Some(corner ) = self.border_radius_obj() {
            Some(corner)
        } else {
            if let Some(val) = self.node.border_radius.clone() {
                let corner = val.clone();
                CornerDeser::from_str(corner).deserialize().ok()    
            } else {
                None
            }
        }
    }

    fn border_radius_obj(&self) -> Option<Corner<f32>> {
        if let Some(val) = self.node.border_radius_obj.clone() {
            let corner = val.clone();
            deserialize_corner(corner).ok()    
        } else {
            None
        }
    }

    fn bottom(&self) -> Option<Units> {
        let prop = &self.node.border_color.clone();
        UiUnit::new(prop.clone()).parse()
    }
    
    fn color(&self) -> Option<Color> {
        let prop = &self.node.color.clone();
        str_to_color(prop)
    }

    fn col_between(&self) -> Option<Units> {
        let prop = &self.node.col_between.clone();
        UiUnit::new(prop.clone()).parse()
    }
    
    fn cursor(&self) -> Option<KCursorIcon> {
        let prop = &self.node.cursor.clone();
        if let Some(val) = prop {
            let icon = to_cursor_icon(val.clone());
            Some(KCursorIcon(icon))
        } else {
            None
        }
    }
    
    fn font(&self) -> Option<String> {
        let prop = &self.node.font.clone();
        Conv::get_prop(prop)
    }

    fn font_size(&self) -> Option<f32> {
        let prop = &self.node.font_size.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_f32()
        } else {
            None
        }        
    }

    fn height(&self) -> Option<Units> {
        let prop = &self.node.height.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn layout_type(&self) -> Option<LayoutType> {
        let prop = &self.node.layout_type.clone();
        if let Some(val) = prop {
            Some(to_layout_type(val.clone()))
        } else {
            None
        }                
    }    

    fn left(&self) -> Option<Units> {
        let prop = &self.node.left.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn line_height(&self) -> Option<f32> {
        let prop = &self.node.line_height.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_f32()
        } else {
            None
        }        
    }

    fn max_height(&self) -> Option<Units> {
        let prop = &self.node.max_height.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn max_width(&self) -> Option<Units> {
        let prop = &self.node.max_width.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn min_height(&self) -> Option<Units> {
        let prop = &self.node.min_height.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn min_width(&self) -> Option<Units> {
        let prop = &self.node.min_width.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn offset_obj(&self) -> Option<Edge<Units>> {
        let optobj = &self.node.offset_obj.clone();
        obj_to_edge_units(optobj)
    }

    fn offset(&self) -> Option<Edge<Units>> {
        let prop = &self.node.offset.clone();
        if let Some(obj) = self.offset_obj() {
            Some(obj)
        } else {
            if let Some(_) = prop {
                to_edge_units(prop.clone())
            } else {
                None
            }            
        }
    }

    fn padding_obj(&self) -> Option<Edge<Units>> {
        let optobj = &self.node.padding_obj.clone();
        obj_to_edge_units(optobj)
    }

    fn padding(&self) -> Option<Edge<Units>> {
        let prop = &self.node.padding.clone();
        if let Some(obj) = self.padding_obj() {
            Some(obj)
        } else {
            if let Some(_) = prop {
                to_edge_units(prop.clone())
            } else {
                None
            }
        }
    }

    fn padding_top(&self) -> Option<Units> {
        let prop = &self.node.padding_top.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn padding_bottom(&self) -> Option<Units> {
        let prop = &self.node.padding_bottom.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn padding_left(&self) -> Option<Units> {
        let prop = &self.node.padding_left.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn padding_right(&self) -> Option<Units> {
        let prop = &self.node.padding_right.clone();
        UiUnit::new(prop.clone()).parse()
    }

    // fn pointer_events(&self) -> PointerEvents {
    //     let prop = &self.node.padding_right.clone();
    //     // UiPointerEvents::new(prop.clone()).parse().unwrap()
    // }
    
    fn position_type(&self) -> Option<KPositionType> {
        let prop = &self.node.position_type.clone();
        if let Some(val) = prop {
            let pt = match val.as_str() {
                "self-directed" => KPositionType::SelfDirected,
                "parent-directed" => KPositionType::ParentDirected,
                _ => KPositionType::SelfDirected,
            };
            Some(pt)
        } else {
            None
        }    
    }

    // fn render_command(&self) -> RenderCommand {
    //     let prop = &self.node.position_type.clone();
    //     UiRenderCommand::new(prop.clone()).parse().unwrap()
    // }

    fn right(&self) -> Option<Units> {
        let prop = &self.node.right.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn row_between(&self) -> Option<Units> {
        let prop = &self.node.row_between.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn top(&self) -> Option<Units> {
        let prop = &self.node.top.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn width(&self) -> Option<Units> {
        let prop = &self.node.width.clone();
        UiUnit::new(prop.clone()).parse()
    }

    fn z_index(&self) -> Option<i32> {
        let prop = &self.node.z_index.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_type::<i32>()
        } else {
            None
        }        
    }
}

impl KStyleDeser {
    pub fn deserialize(&self) -> Result<KStyle, &'static str> {
        let background_color = self.background_color();
        let border = self.border();
        let border_color = self.border_color();
        let border_radius = self.border_radius();
        let bottom = self.bottom();
        let color = self.color();
        let col_between = self.col_between();
        let cursor = self.cursor();
        let font = self.font();
        let font_size = self.font_size();
        let height = self.height();
        let layout_type = self.layout_type();
        let left = self.left();
        let line_height = self.line_height();
        let max_height = self.max_height();
        let max_width = self.max_width();
        let min_height = self.min_height();
        let min_width = self.min_width();
        let offset = self.offset();
        let padding = self.padding();
        let padding_top = self.padding_top();        
        let padding_bottom = self.padding_bottom();        
        let padding_left = self.padding_left();
        let padding_right = self.padding_right();
        let position_type = self.position_type();
        let right = self.right();
        let row_between = self.row_between();
        let top = self.top();
        let width = self.width();        
        let z_index = self.z_index();

        let mut styled = KStyle::default();
        if let Some(val) = background_color {
            styled.background_color = val.into();    
        }
        if let Some(val) = border {
            styled.border = val.into();    
        }
        if let Some(val) = border_color {
            styled.border_color = val.into();    
        }        
        if let Some(val) = border_radius {
            styled.border_radius = val.into();    
        }        
        if let Some(val) = bottom {
            styled.bottom = val.into();    
        }                
        if let Some(val) = color {
            styled.color = val.into();    
        }        
        if let Some(val) = col_between {
            styled.col_between = val.into();    
        }        
        if let Some(val) = cursor {
            styled.cursor = val.into();    
        }    
        if let Some(val) = font {
            styled.font = val.into();    
        }    
        if let Some(val) = font_size {
            styled.font_size = val.into();    
        }    
        if let Some(val) = height {
            styled.height = val.into();    
        }    
        // if let Some(val) = layout_type {
        //     styled.layout_type = val;    
        // }    
        if let Some(val) = left {
            styled.left = val.into();    
        }    
        if let Some(val) = line_height {
            styled.line_height = val.into();    
        }    
        if let Some(val) = max_height {
            styled.max_height = val.into();    
        }    
        if let Some(val) = max_width {
            styled.max_width = val.into();    
        }    
        if let Some(val) = min_height {
            styled.min_height = val.into();    
        }    
        if let Some(val) = min_width {
            styled.min_width = val.into();    
        }    
        if let Some(val) = offset {
            styled.offset = val.into();    
        }    
        if let Some(val) = padding {
            styled.padding = val.into();    
        }    
        if let Some(val) = padding_top {
            styled.padding_top = val.into();    
        }    
        if let Some(val) = padding_bottom {
            styled.padding_bottom = val.into();    
        }    
        if let Some(val) = padding_left {
            styled.padding_left = val.into();    
        }    
        if let Some(val) = padding_right {
            styled.padding_right = val.into();    
        }            
        if let Some(val) = position_type {
            styled.position_type = val.into();    
        }                    
        if let Some(val) = right {
            styled.right = val.into();    
        }            
        if let Some(val) = row_between {
            styled.row_between = val.into();    
        }            
        if let Some(val) = top {
            styled.top = val.into();    
        }            
        if let Some(val) = width {
            styled.width = val.into();    
        }            
        if let Some(val) = z_index {
            styled.z_index = val.into();    
        }            
        Ok(styled)
    }
}
