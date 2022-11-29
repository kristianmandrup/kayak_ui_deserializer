use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, kayak::kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextProps {
    pub alignment: OptStr,
    pub content: OptStr,
    pub font: OptStr,
    pub line_height: OptStr,
    pub show_cursor: OptStr,
    pub size: OptStr,
    pub user_styles: SKStyle,
    pub word_wrap: OptStr
}    