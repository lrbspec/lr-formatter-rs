use derive_builder::Builder;
use getset::{CloneGetters, CopyGetters};

#[derive(CopyGetters, CloneGetters, Debug, Builder)]
pub struct Layer {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get_copy = "pub")]
    index: usize,
    #[builder(default)]
    #[getset(get_clone = "pub")]
    name: Option<String>,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    visible: Option<bool>,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    editable: Option<bool>,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    folder_id: Option<Option<u32>>,
}
