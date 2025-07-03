//! This crate provides safe reading and writing of Line Rider track file formats\
//! Supported formats: LRB (read/write), SOL (read/write), JSON (read/write), TRK (read)
//!
//! # Quickstart
//!
//! # Reading Tracks
//! ```rust
//! ```
//!
//! # Writing Tracks
//! ```rust
//! ```

pub mod formats;
pub mod track;
pub(crate) mod util;

#[cfg(test)]
mod doc_test {
    #[test]
    fn doc_read_example() {
        use crate::formats::trk;
        use std::{fs::File, io::Read};

        // Read a track file
        let mut input_data = Vec::new();
        File::open("../examples/samples/HAM.trk")
            .unwrap()
            .read_to_end(&mut input_data)
            .unwrap();
        let track = trk::read(input_data).unwrap();

        // Use the track
        println!(
            "{} blue lines, {} red lines, {} green lines",
            track.line_group().standard_lines().len(),
            track.line_group().acceleration_lines().len(),
            track.line_group().scenery_lines().len()
        );
    }

    #[test]
    fn doc_write_example() {
        use crate::{formats::trackjson, track::TrackBuilder};
        use std::{fs::File, io::Write};

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
            .unwrap()
    }
}
