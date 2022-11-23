use std::any::Any;

use bevy::prelude::Color;
use kayak_ui::prelude::{Edge, KStyle, Corner, KCursorIcon};
use morphorm::{Units, LayoutType};

use crate::{json_deserializer::UiParseNode, ui_parser::{Conv, UiParser}, ui_color::parse_color, ui_edge::{UiEdge, to_edge_units}, ui_corner::UiCorner, ui_unit::UiUnit, ui_cursor_icon::to_cursor_icon, ui_layout_type::to_layout_type};

pub struct UiStyle {
    node: UiParseNode
}
impl UiStyle {
    pub fn new(node: UiParseNode) -> Self {
        Self {
            node
        }
    }

    fn prop_color(prop: &Option<String>) -> Color {
        let str = Conv::get_prop(prop);
        parse_color(str.as_str())    
    }


    fn background_color(&self) -> Color {
        let prop = &self.node.background_color.clone();
        UiStyle::prop_color(prop)
    }

    fn border(&self) -> Edge<f32> {
        UiEdge::new(self.node.clone()).parse().unwrap()
    }

    fn border_color(&self) -> Color {
        let prop = &self.node.border_color.clone();
        UiStyle::prop_color(prop)
    }

    fn border_radius(&self) -> Corner<f32> {
        UiCorner::new(self.node.clone()).parse().unwrap()
    }

    fn bottom(&self) -> Units {
        let prop = &self.node.border_color.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }
    
    fn color(&self) -> Color {
        let prop = &self.node.color.clone();
        UiStyle::prop_color(prop)
    }

    fn col_between(&self) -> Units {
        let prop = &self.node.col_between.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }
    
    fn cursor(&self) -> KCursorIcon {
        let prop = &self.node.cursor.clone();
        let icon = to_cursor_icon(prop.clone().unwrap());
        KCursorIcon(icon)
    }
    
    fn font(&self) -> String {
        let prop = &self.node.font.clone();
        Conv::get_prop(prop)
    }

    fn font_size(&self) -> f32 {
        let prop = &self.node.font_size.clone();
        Conv(Conv::get_prop(prop)).to_f32()
    }

    fn height(&self) -> Units {
        let prop = &self.node.height.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn layout_type(&self) -> LayoutType {
        let prop = &self.node.layout_type.clone();
        to_layout_type(prop.clone().unwrap())
    }    

    fn left(&self) -> Units {
        let prop = &self.node.left.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn line_height(&self) -> f32 {
        let prop = &self.node.line_height.clone();
        Conv(Conv::get_prop(prop)).to_f32()
    }

    fn max_height(&self) -> Units {
        let prop = &self.node.max_height.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn max_width(&self) -> Units {
        let prop = &self.node.max_width.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn min_height(&self) -> Units {
        let prop = &self.node.min_height.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn min_width(&self) -> Units {
        let prop = &self.node.min_width.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn offset(&self) -> Edge<Units> {
        let prop = &self.node.offset.clone();
        to_edge_units(prop.clone())
    }

    fn padding(&self) -> Edge<Units> {
        let prop = &self.node.padding.clone();
        to_edge_units(prop.clone())
    }

    fn padding_top(&self) -> Units {
        let prop = &self.node.padding_top.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn padding_bottom(&self) -> Units {
        let prop = &self.node.padding_bottom.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn padding_left(&self) -> Units {
        let prop = &self.node.padding_left.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }

    fn padding_right(&self) -> Units {
        let prop = &self.node.padding_right.clone();
        UiUnit::new(prop.clone()).parse().unwrap()
    }
}

impl UiParser for UiStyle {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str> {
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
        
        let widget = KStyle {
            background_color: background_color.into(),
            border: border.into(),
            border_color: border_color.into(),
            border_radius: border_radius.into(),
            bottom: bottom.into(),
            color: color.into(),
            col_between: col_between.into(),
            cursor: cursor.into(),
            font: font.into(),
            font_size: font_size.into(),
            height: height.into(),
            layout_type: layout_type.into(),
            left: left.into(),
            line_height: line_height.into(),
            max_height: max_height.into(),
            max_width: max_width.into(),
            min_height: min_height.into(),
            min_width: min_width.into(),
            offset: offset.into(),
            padding: padding.into(),
            padding_top: padding_top.into(),
            padding_bottom: padding_bottom.into(),
            padding_left: padding_left.into(),
            padding_right: padding_right.into(),
            ..Default::default()
        };
        Ok(Box::new(widget))
    }
}



// pub struct KStyle {
//     /// The background color of this widget
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub background_color : StyleProp<Color>,
//     /// The color of the border around this widget
//     ///
//     /// Currently, this controls all border sides.
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub border_color: StyleProp<Color>,
//     /// The radius of the corners (in pixels)
//     ///
//     /// The order is (Top, Right, Bottom, Left).
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`] and [`RenderCommand::Image`]
//     pub border_radius: StyleProp<Corner<f32>>,
//     /// The widths of the borders (in pixels)
//     ///
//     /// The order is (Top, Right, Bottom, Left).
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub border: StyleProp<Edge<f32>>,
//     /// The distance between the bottom edge of this widget and the bottom edge of its containing widget
//     pub bottom: StyleProp<Units>,
//     /// The text color for this widget
//     ///
//     /// This property defaults to [`StyleProp::Inherit`] meaning that setting this field to some value will
//     /// cause all descendents to receive that value, up to the next set value.
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Text`]
//     pub color: StyleProp<Color>,
//     /// The spacing between child widgets along the horizontal axis
//     pub col_between: StyleProp<Units>,
//     /// The cursor icon to display when hovering this widget
//     pub cursor: StyleProp<KCursorIcon>,
//     /// The font name for this widget
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub font: StyleProp<String>,
//     /// The font size for this widget, in pixels
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub font_size: StyleProp<f32>,
//     /// The height of this widget
//     pub height: StyleProp<Units>,
//     /// The layout method for children of this widget
//     pub layout_type: StyleProp<LayoutType>,
//     /// The distance between the left edge of this widget and the left edge of its containing widget
//     pub left: StyleProp<Units>,
//     /// The line height for this widget, in pixels
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub line_height: StyleProp<f32>,
//     /// The maximum height of this widget
//     pub max_height: StyleProp<Units>,
//     /// The maximum width of this widget
//     pub max_width: StyleProp<Units>,
//     /// The minimum height of this widget
//     pub min_height: StyleProp<Units>,
//     /// The minimum width of this widget
//     pub min_width: StyleProp<Units>,
//     /// The positional offset from this widget's default position
//     ///
//     /// This property has lower precedence than its more specific counterparts
//     /// ([`top`](Self::top), [`right`](Self::right), [`bottom`](Self::bottom), and [`left`](Self::left)),
//     /// allowing it to be overridden.
//     ///
//     /// For widgets with a [`position_type`](Self::position_type) of [`PositionType`](PositionType::ParentDirected)
//     /// this acts like margin around the widget. For [`PositionType`](PositionType::SelfDirected) this
//     /// acts as the actual position from the parent.
//     pub offset: StyleProp<Edge<Units>>,
//     /// The inner padding between the edges of this widget and its children
//     ///
//     /// This property has lower precedence than its more specific counterparts
//     /// ([`padding_top`](Self::padding_top), [`padding_right`](Self::padding_right),
//     /// [`padding_bottom`](Self::padding_bottom), and [`padding_left`](Self::padding_left)), allowing it
//     /// to be overridden.
//     ///
//     /// A child with their own padding properties set to anything other than [`Units::Auto`] will
//     /// override the padding set by this widget.
//     pub padding: StyleProp<Edge<Units>>,
//     /// The inner padding between the bottom edge of this widget and its children
//     ///
//     /// A child with their own `bottom` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_bottom: StyleProp<Units>,
//     /// The inner padding between the left edge of this widget and its children
//     ///
//     /// A child with their own `left` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_left: StyleProp<Units>,
//     /// The inner padding between the right edge of this widget and its children
//     ///
//     /// A child with their own `right` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_right: StyleProp<Units>,
//     /// The inner padding between the top edge of this widget and its children
//     ///
//     /// A child with their own `top` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_top: StyleProp<Units>,
//     /// Controls how the pointer interacts with the widget
//     ///
//     /// This can be used to block pointer events on itself and/or its children if needed, allowing
//     /// the event to "pass through" to widgets below.
//     pub pointer_events: StyleProp<PointerEvents>,
//     /// The position type of the widget relative to its parent
//     pub position_type: StyleProp<KPositionType>,
//     /// The render method for this widget
//     ///
//     /// This controls what actually gets rendered and how it's rendered.
//     pub render_command: StyleProp<RenderCommand>,
//     /// The distance between the right edge of this widget and the right edge of its containing widget
//     pub right: StyleProp<Units>,
//     /// The spacing between child widgets along the vertical axis
//     pub row_between: StyleProp<Units>,
//     /// The distance between the top edge of this widget and the top edge of its containing widget
//     pub top: StyleProp<Units>,
//     /// The width of this widget
//     pub width: StyleProp<Units>,
//     /// The z-index relative to it's parent.
//     pub z_index: StyleProp<i32>,
// }
// }