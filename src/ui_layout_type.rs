use morphorm::LayoutType;

struct UiLayoutType;

pub fn to_layout_type(str: String) -> LayoutType {
    match str.as_str() {
        "row" => LayoutType::Row,
        "column" => LayoutType::Column,
        "grid" => LayoutType::Grid,
        _ => panic!("Invalid layout type")
    }
}
