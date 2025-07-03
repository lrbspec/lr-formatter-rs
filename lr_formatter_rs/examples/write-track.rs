use lr_formatter_rs::{formats::trackjson, track::TrackBuilder};
use std::{fs::File, io::Write};

fn main() {
    // Construct a new track
    let mut track_builder = TrackBuilder::new();
    track_builder.metadata().title("New track");
    track_builder.metadata().artist("Me");
    let track = track_builder.build().unwrap();

    // Write the track
    let output_data = trackjson::write(&track).unwrap();
    File::create("new_track.track.json")
        .unwrap()
        .write_all(&output_data)
        .unwrap();
}
