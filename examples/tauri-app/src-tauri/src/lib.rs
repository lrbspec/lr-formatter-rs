use lr_formatter_rs::{
    convert,
    formats::{sol::get_track_count, Format},
};

#[tauri::command]
fn convert_files(
    file_bytes: Vec<u8>,
    from_format: String,
    to_format: String,
    sol_index: u32,
) -> Result<Vec<u8>, String> {
    let from = parse_format(&from_format, Some(sol_index))?;
    let to = parse_format(&to_format, None)?;

    convert(&file_bytes, from, to).map_err(|e| format!("Conversion failed: {}", e))
}

#[tauri::command]
fn get_max_sol_index(file_bytes: Vec<u8>) -> u32 {
    return get_track_count(&file_bytes) - 1;
}

fn parse_format(fmt: &str, sol_index: Option<u32>) -> Result<Format, String> {
    match fmt.to_uppercase().as_str() {
        "JSON" => Ok(Format::TrackJson),
        "LRB" => Ok(Format::LRB),
        "TRK" => Ok(Format::TRK),
        "SOL" => Ok(Format::SOL(sol_index)),
        _ => Err(format!("Unknown format: {}", fmt)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_files, get_max_sol_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
