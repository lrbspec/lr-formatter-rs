use crate::internal::{GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonLine {
  id: u32,
  #[serde(rename = "type")]
  line_type: u8,
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  flipped: bool,
  #[serde(rename = "leftExtended")]
  left_ext: Option<bool>,
  #[serde(rename = "rightExtended")]
  right_ext: Option<bool>,
  multiplier: Option<f64>,
  width: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonLayer {
  id: u32,
  #[serde(rename = "type")]
  layer_type: u8,
  name: String,
  visible: bool,
  editable: bool,
  #[serde(rename = "folderId")]
  folder_id: Option<u32>,
  size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRider {
  #[serde(rename = "startPosition")]
  start_pos: Vec2,
  #[serde(rename = "startVelocity")]
  start_vel: Vec2,
  remountable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTrack {
    label: String,
    creator: String,
    description: String,
    duration: u32,
    version: String,
    #[serde(rename = "startPosition")]
    start_pos: Vec2,
    lines: Vec<JsonLine>,
    layers: Vec<JsonLayer>,
    riders: Vec<JsonRider>,
    script: String,
}

pub fn parse_track_json(json_str: &str) -> Result<InternalTrackFormat, Box<dyn std::error::Error>> {
    let track: JsonTrack = serde_json::from_str(json_str)?;

    let grid_version = match track.version.as_str() {
      "6.0" => GridVersion::V6_0,
      "6.1" => GridVersion::V6_1,
      "6.2" => GridVersion::V6_2,
      other => panic!("[ERROR] Invalid grid version {other} when parsing json!")
    };

    let mut scenery_lines = Vec::<SceneryLine>::new(); 
    let mut simulation_lines = Vec::<SimulationLine>::new(); 

    for line in track.lines {
      let line_type = match line.line_type {
        0 => LineType::BLUE,
        1 => LineType::RED,
        2 => LineType::GREEN,
        other => panic!("[ERROR] Json line had invalid line type {other}!"),
      };

      let base_line = Line {
        id: line.id,
        x1: line.x1,
        y1: line.y1,
        x2: line.x2,
        y2: line.y2,
        line_type: line_type,
      };

      if line.line_type == 2 {
        scenery_lines.push(SceneryLine {
          base_line,
          width: None,
        });
      } else {
        simulation_lines.push(SimulationLine {
          base_line,
          flipped: line.flipped,
          left_extension: line.left_ext.expect("[ERROR] Json simulation line did not have left extension!"),
          right_extension: line.right_ext.expect("[ERROR] Json simulation line did not have right extension!"),
          multiplier: None,
        });
      }
    }

    Ok(InternalTrackFormat {
      title: track.label,
      grid_version,
      start_position: track.start_pos,
      scenery_lines,
      simulation_lines
    })
}

pub fn write_track_json(
    internal: &InternalTrackFormat,
) -> Result<String, Box<dyn std::error::Error>> {
    let track = JsonTrack {
        label: internal.title.clone(),
    };

    Ok(serde_json::to_string_pretty(&track)?)
}
