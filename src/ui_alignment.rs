use kayak_ui::prelude::Alignment;

pub fn to_alignment(align: String) -> Alignment {
    match align.to_lowercase().as_str() {
        "start" => Alignment::Start,
        "middle" => Alignment::Middle,
        "end" => Alignment::End,
        _ => Alignment::Start
    }
}
