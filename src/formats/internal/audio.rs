#[derive(Debug, Clone)]
pub struct Audio {
    // File name of the audio relative to the directory the track file was located in during save
    pub file_name: String,
    // Offset (in seconds) to delay the start of the audio
    pub offset_until_start: f64,
}
