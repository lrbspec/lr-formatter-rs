use super::{JsonLayer, JsonLine, JsonRider, JsonTrack, Vec2};
use crate::{
    TrackWriteError,
    formats::internal::{GridVersion, InternalTrackFormat, LineType},
};

pub fn write(internal: &InternalTrackFormat) -> Result<String, TrackWriteError> {
    let version = match internal.grid_version {
        GridVersion::V6_0 => String::from("6.0"),
        GridVersion::V6_1 => String::from("6.1"),
        GridVersion::V6_2 => String::from("6.2"),
    };

    let mut lines = Vec::<JsonLine>::new();

    for line in &internal.simulation_lines {
        let line_type = if line.base_line.line_type == LineType::BLUE {
            0u8
        } else {
            1u8
        };

        lines.push(JsonLine {
            id: line.base_line.id,
            line_type,
            x1: line.base_line.x1,
            y1: line.base_line.y1,
            x2: line.base_line.x2,
            y2: line.base_line.y2,
            flipped: Some(line.flipped),
            left_ext: Some(line.left_extension),
            right_ext: Some(line.right_extension),
            extended: None,
            multiplier: line.multiplier,
            width: None,
        });
    }

    for line in &internal.scenery_lines {
        lines.push(JsonLine {
            id: line.base_line.id,
            line_type: 2,
            x1: line.base_line.x1,
            y1: line.base_line.y1,
            x2: line.base_line.x2,
            y2: line.base_line.y2,
            flipped: None,
            left_ext: None,
            right_ext: None,
            extended: None,
            multiplier: None,
            width: line.width,
        });
    }

    let start_pos = Vec2 {
        x: internal.start_position.x,
        y: internal.start_position.y,
    };

    let track = JsonTrack {
        label: internal.title.clone(),
        version,
        start_pos,
        lines: Some(lines),
        creator: Some(internal.artist.clone()),
        description: Some(internal.description.clone()),
        duration: Some(internal.duration),
        script: Some(internal.script.clone()),
        layers: Some(Vec::<JsonLayer>::new()),
        riders: Some(Vec::<JsonRider>::new()),
        line_array: None,
        time_based_triggers: None,
        // TODO
        start_zoom: None,
        zero_start: None,
        line_based_triggers: None,
        line_color_blue: None,
        line_color_green: None,
        line_color_red: None,
        background_color_blue: None,
        background_color_green: None,
        background_color_red: None,
        gravity_well_size: None,
        x_gravity: None,
        y_gravity: None,
    };

    let track_string = serde_json::to_string(&track).map_err(|err| TrackWriteError::Other {
        message: format!("Failed to serialize json track: {}", err),
    })?;

    Ok(track_string)
}
