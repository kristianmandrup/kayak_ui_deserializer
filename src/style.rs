use std::any::Any;

use bevy::prelude::Color;
use kayak_ui::prelude::{Edge, KStyle, Corner};

use crate::{json_deserializer::UiParseNode, ui_parser::{Conv, UiParser}, color::parse_color, edge::UiEdge, corner::UiCorner};

pub struct UiStyles {
    node: UiParseNode
}
impl UiStyles {
    fn new(node: UiParseNode) -> Self {
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
        UiStyles::prop_color(prop)
    }

    fn border(&self) -> Edge<f32> {
        UiEdge::new(self.node.clone()).parse().unwrap()
    }

    // pub border_color: StyleProp<Color>,
    // border_radius: StyleProp<Corner<f32>>

    fn border_color(&self) -> Color {
        let prop = &self.node.border_color.clone();
        UiStyles::prop_color(prop)
    }

    fn border_radius(&self) -> Corner<f32> {
        let prop = &self.node.border_color.clone();
        UiCorner::new(self.node.clone()).parse().unwrap()
    }

    // bottom

}

impl UiParser for UiStyles {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str> {
        let color = self.background_color();
        let border = self.border();
        let border_color = self.border_color();
        let border_radius = self.border_radius();
        let widget = KStyle {
            background_color: color.into(),
            border: border.into(),
            border_color: border_color.into(),
            border_radius: border_radius.into(),
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