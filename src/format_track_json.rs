use serde::{Deserialize, Serialize};
use crate::internal::InternalFormat;

#[derive(Serialize, Deserialize, Debug)]
struct TrackJson {
    label: String,
}

pub fn parse_track_json(json_str: &str) -> Result<InternalFormat, Box<dyn std::error::Error>> {
    let track: TrackJson = serde_json::from_str(json_str)?;
    Ok(InternalFormat {
        title: track.label,
    })
}

pub fn write_track_json(internal: &InternalFormat) -> Result<String, Box<dyn std::error::Error>> {
    let track = TrackJson {
        label: internal.title.clone(),
    };

    Ok(serde_json::to_string_pretty(&track)?)
}
