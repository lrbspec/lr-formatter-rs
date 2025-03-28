use crate::internal::InternalTrackFormat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TrackJson {
    label: String,
}

pub fn parse_track_json(json_str: &str) -> Result<InternalTrackFormat, Box<dyn std::error::Error>> {
    let track: TrackJson = serde_json::from_str(json_str)?;
    Ok(InternalTrackFormat { title: track.label })
}

pub fn write_track_json(
    internal: &InternalTrackFormat,
) -> Result<String, Box<dyn std::error::Error>> {
    let track = TrackJson {
        label: internal.title.clone(),
    };

    Ok(serde_json::to_string_pretty(&track)?)
}
