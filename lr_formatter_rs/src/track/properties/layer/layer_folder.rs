use derive_builder::Builder;
use getset::{CloneGetters, CopyGetters};

#[derive(Debug, CopyGetters, CloneGetters, Builder)]
pub struct LayerFolder {
    #[getset(get_copy = "pub")]
    id: u32,
    #[getset(get_copy = "pub")]
    index: usize,
    #[builder(default = Some("".to_string()))]
    #[getset(get_clone = "pub")]
    name: Option<String>,
    #[builder(default = Some(true))]
    #[getset(get_copy = "pub")]
    visible: Option<bool>,
    #[builder(default = Some(true))]
    #[getset(get_copy = "pub")]
    editable: Option<bool>,
    #[builder(default = Some(0))]
    #[getset(get_copy = "pub")]
    size: Option<u32>,
}
