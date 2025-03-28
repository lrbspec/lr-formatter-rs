pub mod format_internal;
pub mod format_lrb;
pub mod format_track_json;

use anyhow::Result;
use format_lrb::{parse_lrb, write_lrb};
use format_track_json::{parse_track_json, write_track_json};

pub enum Format {
    TrackJson,
    LRB,
}

pub fn convert(input: &[u8], from: Format, to: Format) -> Result<Vec<u8>> {
    let internal_format = match from {
        Format::TrackJson => {
            let input_str = String::from_utf8(input.to_vec())?;
            parse_track_json(&input_str)?
        }
        Format::LRB => parse_lrb(input)?,
    };

    let output_bytes = match to {
        Format::TrackJson => {
            let json_str = write_track_json(&internal_format)?;
            json_str.into_bytes()
        }
        Format::LRB => write_lrb(&internal_format)?,
    };

    Ok(output_bytes)
}
