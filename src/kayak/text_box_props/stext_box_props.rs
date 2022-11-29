use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextBoxProps {
    pub disabled: OptStr,
    pub placeholder: OptStr,
    pub value: OptStr,
}
