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


// pub enum LayoutType {
//     /// Stack child elements horizontally
//     Row,
//     /// Stack child elements vertically
//     Column,
//     /// Position child elements into specified rows and columns
//     Grid,
// }
