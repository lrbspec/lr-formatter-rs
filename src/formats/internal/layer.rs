use crate::internal::rgb_color::RGBColor;

// TODO: Refactor this into two separate structs, with an "index" attribute
#[derive(Debug, Clone)]
pub enum Layer {
    Layer {
        id: u32,
        name: String,
        color: RGBColor,
        visible: bool,
        editable: bool,
        folder_id: Option<u32>,
    },
    Folder {
        id: u32,
        name: String,
        visible: bool,
        editable: bool,
        size: u32,
    },
}
