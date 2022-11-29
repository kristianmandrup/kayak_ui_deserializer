use kayak_ui::prelude::LayoutType;


pub fn to_layout_type(str: String) -> LayoutType {
    match str.as_str() {
        "row" => LayoutType::Row,
        "column" => LayoutType::Column,
        "grid" => LayoutType::Grid,
        _ => panic!("Invalid layout type")
    }
}

pub fn layout_type_to_str(lt: LayoutType) -> String {
    match lt {
        LayoutType::Row => "row".to_string(),
        LayoutType::Column => "column".to_string(),
        LayoutType::Grid => "grid".to_string(),
    }
}
