pub mod format_lrb;
pub mod format_track_json;
pub mod format_internal;

use format_lrb::{parse_lrb, write_lrb};
use format_track_json::{parse_track_json, write_track_json};

pub enum Format {
    TrackJson,
    LRB,
}

pub fn convert(input: &[u8], from: Format, to: Format) -> Result<Vec<u8>, String> {
    let internal_format = match from {
        Format::TrackJson => {
            let input_str = String::from_utf8(input.to_vec()).map_err(|e| e.to_string())?;
            parse_track_json(&input_str).map_err(|e| e.to_string())?
        }
        Format::LRB => parse_lrb(input).map_err(|e| e.to_string())?,
    };

    match to {
        Format::TrackJson => {
            let json_str = write_track_json(&internal_format).map_err(|e| e.to_string())?;
            Ok(json_str.into_bytes())
        }
        Format::LRB => write_lrb(&internal_format).map_err(|e| e.to_string()),
    }
}
