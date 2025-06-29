use derive_builder::Builder;
use getset::Getters;

#[derive(Debug, Getters, Builder)]
#[getset(get = "pub")]
pub struct LayerFolder {
    id: u32,
    index: usize,
    #[builder(setter(into, strip_option), default)]
    name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    visible: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    editable: Option<bool>,
    #[builder(setter(into, strip_option), default)]
    size: Option<u32>,
}
