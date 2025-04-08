pub mod formats;

use anyhow::Result;
use formats::lrb;
use formats::trackjson;

pub enum Format {
    TrackJson,
    LRB,
}

pub fn convert(input: &[u8], from: Format, to: Format) -> Result<Vec<u8>> {
    let internal_format = match from {
        Format::TrackJson => {
            let input_str = String::from_utf8(input.to_vec())?;
            trackjson::reader::read(&input_str)?
        }
        Format::LRB => lrb::reader::read(input)?,
    };

    let output_bytes = match to {
        Format::TrackJson => {
            let json_str = trackjson::writer::write(&internal_format)?;
            json_str.into_bytes()
        }
        Format::LRB => lrb::writer::write(&internal_format)?,
    };

    Ok(output_bytes)
}
