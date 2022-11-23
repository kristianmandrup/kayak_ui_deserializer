use std::fmt::Debug;
use std::str::FromStr;
use std::{any::Any};
use morphorm::Units;

use crate::json_deserializer::{UiParseNode, };
use crate::ui_unit::UiUnit; // use morphorm::Cache;

pub trait UiParser {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str>;
}

// pub struct Rect {
//     pub posx: f32,
//     pub posy: f32,
//     pub width: f32,
//     pub height: f32,
//     pub z_index: f32,
// }

// pub struct Space {
//     pub left: f32,
//     pub right: f32,
//     pub top: f32,
//     pub bottom: f32,
// }

// pub struct Size {
//     pub width: f32,
//     pub height: f32,
// }

// pub struct Layout {
//     /// width of the component
//     pub width: f32,
//     /// height of the component
//     pub height: f32,
//     /// x-coordinates of the component
//     pub x: f32,
//     /// y-coordinates of the component
//     pub y: f32,
//     /// z-coordinates of the component
//     pub z: f32,
// }

// pub struct BackgroundBundle {
//     pub background: Background,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }

// pub struct KayakAppBundle {
//     pub app: KayakApp,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub widget_name: WidgetName,
// }


/// A generic widget
/// You can consider this to kind behave like a div in html
/// Accepts: KStyle, OnEvent, and KChildren.
// #[derive(Bundle)]
// pub struct ElementBundle {
//     pub element: Element,
//     pub styles: KStyle,
//     pub on_event: OnEvent,
//     pub children: KChildren,
//     pub widget_name: WidgetName,
// }

// pub struct TextBoxBundle {
//     pub text_box: TextBoxProps,
//     pub styles: KStyle,
//     pub on_event: OnEvent,
//     pub on_layout: OnLayout,
//     pub on_change: OnChange,
//     pub focusable: Focusable,
//     pub widget_name: WidgetName,
// }

// pub struct TextProps {
//     /// The string to display
//     pub content: String,
//     /// The name of the font to use
//     ///
//     /// The given font must already be loaded into the [`KayakContext`](kayak_core::KayakContext)
//     pub font: Option<String>,
//     /// The height of a line of text (currently in pixels)
//     pub line_height: Option<f32>,
//     /// If true, displays the default text cursor when hovered.
//     ///
//     /// This _will_ override the `cursor` style.
//     pub show_cursor: bool,
//     /// The font size (in pixels)
//     ///
//     /// Negative values have no effect
//     pub size: f32,
//     /// Text alignment.
//     pub alignment: Alignment,
//     /// Custom styles to pass in.
//     pub user_styles: KStyle,
//     /// Basic word wrapping.
//     /// Defautls to true
//     pub word_wrap: bool,
// }

// pub struct TextureAtlasProps {
//     /// The handle to image
//     pub handle: Handle<Image>,
//     /// The position of the tile (in pixels)
//     pub position: Vec2,
//     /// The size of the tile (in pixels)
//     pub tile_size: Vec2,
// }

// pub struct KWindow {
//     /// If true, allows the window to be draggable by its title bar
//     pub draggable: bool,
//     /// The initial position at which to display the window in pixels
//     pub initial_position: Vec2,
//     /// The size of the window in pixels
//     pub size: Vec2,
//     /// The text to display in the window's title bar
//     pub title: String,
//     /// Styles for the main window quad.
//     pub window_styles: KStyle,
//     /// A set of styles to apply to the children element wrapper.
//     pub children_styles: KStyle,
// }



// pub struct KImage(pub Handle<bevy::prelude::Image>);
// pub struct KImageBundle {
//     pub image: KImage,
//     pub styles: KStyle,
//     pub widget_name: WidgetName,
// }



// #[derive(Bundle)]
// pub struct ClipBundle {
//     pub clip: Clip,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub widget_name: WidgetName,
// }


// pub struct KButton {
//     pub text: String,
//     pub user_styles: KStyle,
// }

// /// Default button widget
// /// Accepts an OnEvent component
// #[derive(Bundle)]
// pub struct KButtonBundle {
//     pub button: KButton,
//     pub styles: KStyle,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }




// KChildren
// inner: Vec<Entity>

// - TextProps
//     - `content`: The string to display
//     - `font`: The name of the font to use 
//     - `line_height`: The height of a line of text (currently in pixels). Defaults to font size * 1.2 which is the firefox default method of calculating line height.
//     - `show_cursor`: If true, displays the default text cursor when hovered.
//     - `size`: The font size (in pixels)
//     - `alignment`: Text alignment.
//     - `user_styles`: Specific styles applied directly to the text itself.
//     - `word_wrap`: Wraps the words if said text would overflow it's parent.

pub struct Conv(pub String);

impl Conv {
    pub fn option_str(&self) -> Option<String> {
        let str = self.0.clone();
        if str.is_empty() { None } else { Some(str) }
    }

    pub fn option_f32(&self) -> Option<f32> {
        let str = self.0.clone();
        if str.is_empty() { None } else { Some(self.to_f32()) }
    }

    pub fn to_f32(&self) -> f32 {
        let str = self.0.clone();
        str.trim().parse::<f32>().unwrap()    
    }

    pub fn to_type<T: FromStr + Debug + Default>(&self)  -> T 
    {
        let str = self.0.clone();
        str.trim().parse::<T>().unwrap_or_default()
    }


    pub fn get_prop(prop: &Option<String>) -> String {
        prop.to_owned().unwrap_or("".to_string())
    }

    pub fn to_bool(&self) -> bool {
        let str = self.0.clone();
        str.trim().parse().unwrap()
    }
}



//     - `font`: The name of the font to use 
//     - `line_height`: The height of a line of text (currently in pixels). Defaults to font size * 1.2 which is the firefox default method of calculating line height.
//     - `show_cursor`: If true, displays the default text cursor when hovered.
//     - `size`: The font size (in pixels)
//     - `alignment`: Text alignment.
//     - `user_styles`: Specific styles applied directly to the text itself.
//     - `word_wrap`: Wraps the words if said text would overflow it's parent.


pub struct UiNode {
    pub width: Units
}
impl UiNode {
    // fn new(width: UiNodeUnit) -> Self {
    //     Self {
    //         width
    //     }
    // }
}

pub fn build_text_widget(ui: UiParseNode) -> Result<UiNode, &'static str>  {
    if let Ok(unit) = UiUnit::new(ui.width).parse() {
        Ok(UiNode {
            width: unit
        })
    } else {
        Err("bad Text Widget")        
    }
    
}

