pub mod internal;
pub mod lrb;
pub mod sol;
pub mod trackjson;
pub mod trk;

pub enum Format {
    TrackJson,
    LRB,
    TRK,
    SOL(Option<u32>),
}
