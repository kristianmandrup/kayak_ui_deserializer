use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, kayak::kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SWindow {
    /// If true, allows the window to be draggable by its title bar
    pub draggable: OptStr,
    /// The initial position at which to display the window in pixels
    pub initial_position: Option<Vec<OptStr>>,
    /// The size of the window in pixels
    pub size: Option<Vec<OptStr>>,
    /// The text to display in the window's title bar
    pub title: Option<String>,
    /// Styles for the main window quad.
    pub window_styles: Option<SKStyle>,
    /// A set of styles to apply to the children element wrapper.
    pub children_styles: Option<SKStyle>,
}