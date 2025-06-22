mod formats;
pub use formats::{Format, internal, lrb, sol, trackjson, trk};
pub(crate) mod util;

use anyhow::Result;

pub fn convert(input: &[u8], from: Format, to: Format) -> Result<Vec<u8>> {
    let internal_format = match from {
        Format::TrackJson => {
            let input_str = String::from_utf8(input.to_vec())?;
            trackjson::read(&input_str)?
        }
        Format::LRB => lrb::read(input)?,
        Format::TRK => trk::read(input)?,
        Format::SOL(track_index) => sol::read(input, track_index)?,
    };

    let output_bytes = match to {
        Format::TrackJson => {
            let json_str = trackjson::write(&internal_format)?;
            Ok(json_str.into_bytes())
        }
        Format::LRB => lrb::write(&internal_format),
        Format::SOL(_) => sol::write(&internal_format),
        _ => Err(anyhow::anyhow!(
            "Unsupported to format. Must be one of: trackjson, lrb, sol",
        )),
    };

    output_bytes
}
