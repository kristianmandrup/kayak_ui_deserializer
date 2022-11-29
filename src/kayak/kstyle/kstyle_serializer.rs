use kayak_ui::prelude::KStyle;

use crate::{bevy::{color::color_deser::color_to_str, corner::corner_deser::corner_to_str}, serialized::OptStr, morphorm::{units::units_to_str, layout_type::layout_type_to_str}, kayak::edge::edge_deser::{edge_to_str, edge_units_to_str}};

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

    fn background_color(&self) -> OptStr {
        let color = self.node.background_color.resolve();
        Some(color_to_str(color))
    }

    fn border_color(&self) -> OptStr {
        let color = self.node.border_color.resolve();
        Some(color_to_str(color))
    }

    fn border_radius(&self) -> OptStr {
        let prop = self.node.border_radius.resolve();
        Some(corner_to_str(prop))
    }

    fn border(&self) -> OptStr {
        let prop = self.node.border.resolve();
        Some(edge_to_str(prop))
    }
    
    fn bottom(&self) -> OptStr {
        let prop = self.node.bottom.resolve();
        Some(units_to_str(prop))
    }

    fn color(&self) -> OptStr {
        let color = self.node.color.resolve();
        Some(color_to_str(color))
    }

    fn col_between(&self) -> OptStr {
        let prop = self.node.col_between.resolve();
        Some(units_to_str(prop))
    }

    // cursor
    
    fn font(&self) -> OptStr {
        let font = self.node.font.resolve();
        Some(font)
    }

    fn font_size(&self) -> OptStr {
        let font_size = self.node.font_size.resolve();
        Some(font_size.to_string())
    }

    fn height(&self) -> OptStr {
        let prop = self.node.height.resolve();
        Some(units_to_str(prop))
    }
    
    fn layout_type(&self) -> OptStr {
        let prop = self.node.layout_type.resolve();
        Some(layout_type_to_str(prop))
    }
    
    fn left(&self) -> OptStr {
        let prop = self.node.left.resolve();
        Some(units_to_str(prop))
    }    
    
    fn line_height(&self) -> OptStr {
        let line_height = self.node.line_height.resolve();
        Some(line_height.to_string())
    }

    fn max_height(&self) -> OptStr {
        let prop = self.node.max_height.resolve();
        Some(units_to_str(prop))
    }    

    fn max_width(&self) -> OptStr {
        let prop = self.node.max_width.resolve();
        Some(units_to_str(prop))
    }    

    fn min_height(&self) -> OptStr {
        let prop = self.node.min_height.resolve();
        Some(units_to_str(prop))
    }    

    fn min_width(&self) -> OptStr {
        let prop = self.node.min_width.resolve();
        Some(units_to_str(prop))
    }    
    
    fn offset(&self) -> OptStr {
        let prop = self.node.offset.resolve();
        Some(edge_units_to_str(prop))
    }    
    
    fn padding(&self) -> OptStr {
        let prop = self.node.padding.resolve();
        Some(edge_units_to_str(prop))
    }    

    fn padding_bottom(&self) -> OptStr {
        let prop = self.node.padding_bottom.resolve();
        Some(units_to_str(prop))
    }    

    fn padding_left(&self) -> OptStr {
        let prop = self.node.padding_left.resolve();
        Some(units_to_str(prop))
    }    

    fn padding_right(&self) -> OptStr {
        let prop = self.node.padding_right.resolve();
        Some(units_to_str(prop))
    }    

    fn padding_top(&self) -> OptStr {
        let prop = self.node.padding_top.resolve();
        Some(units_to_str(prop))
    }    

    fn right(&self) -> OptStr {
        let prop = self.node.right.resolve();
        Some(units_to_str(prop))
    }    

    fn row_between(&self) -> OptStr {
        let prop = self.node.col_between.resolve();
        Some(units_to_str(prop))
    }

    fn top(&self) -> OptStr {
        let prop = self.node.top.resolve();
        Some(units_to_str(prop))
    }    

    fn width(&self) -> OptStr {
        let prop = self.node.height.resolve();
        Some(units_to_str(prop))
    }

    fn z_index(&self) -> OptStr {
        let line_height = self.node.z_index.resolve();
        Some(line_height.to_string())
    }

    pub fn serialize(&self) -> Result<SKStyle, &'static str> {
        let background_color = self.background_color();
        let border_color = self.border_color();
        let border_radius = self.border_radius();
        let border = self.border();
        let bottom = self.bottom();
        let color = self.color();
        let col_between = self.col_between();
        let font = self.font();
        let font_size = self.font_size();
        let height = self.height();
        let layout_type = self.layout_type();
        let left = self.left();
        let line_height = self.line_height();
        let max_height = self.max_height();
        let min_height = self.min_height();
        let max_width = self.max_width();
        let min_width = self.min_width();
        let offset = self.offset();
        let padding = self.padding();
        let padding_top = self.padding_top();
        let padding_right = self.padding_right();
        let padding_bottom = self.padding_bottom();
        let padding_left = self.padding_left();
        let right = self.right();
        let row_between = self.row_between();
        let top = self.top();
        let row_between = self.row_between();
        let width = self.width();
        let z_index = self.z_index();

        let kstyle = SKStyle {            
            background_color,
            border_color,
            border_radius,
            border,
            bottom,
            color,
            col_between,
            font,
            font_size,
            height,
            layout_type,
            left,
            line_height,
            min_height,
            min_width,
            max_height,
            max_width,
            offset,
            padding_top,
            padding_right,
            padding_bottom,
            padding_left,
            right,
            row_between,
            top,
            width,
            z_index,
            ..Default::default() 
        };
        Ok(kstyle)
    }    
}