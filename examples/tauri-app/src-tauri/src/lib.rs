use lr_formatter_rs::{convert,formats::Format};

#[tauri::command]
fn convert_files(
    file_bytes: Vec<u8>,
    from_format: String,
    to_format: String,
) -> Result<Vec<u8>, String> {
    let from = parse_format(&from_format)?;
    let to = parse_format(&to_format)?;

    convert(&file_bytes, from, to).map_err(|e| format!("Conversion failed: {}", e))
}

fn parse_format(fmt: &str) -> Result<Format, String> {
    match fmt.to_uppercase().as_str() {
        "JSON" => Ok(Format::TrackJson),
        "LRB" => Ok(Format::LRB),
        "TRK" => Ok(Format::TRK),
        "SOL" => Ok(Format::SOL(None)),
        _ => Err(format!("Unknown format: {}", fmt)),
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
