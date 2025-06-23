use lr_formatter_rs::{lrb, sol, trackjson, trk};

#[tauri::command]
fn convert_files(
    file_bytes: Vec<u8>,
    from_format: String,
    to_format: String,
    sol_index: u32,
) -> Result<Vec<u8>, String> {
    let internal_format = match from_format.to_uppercase().as_str() {
        "JSON" => {
            let input_str = String::from_utf8(file_bytes.to_vec())
                .map_err(|e| format!("Failed to stringify file: {}", e))?;
            trackjson::read(&input_str).map_err(|e| format!("Failed to parse: {}", e))?
        }
        "LRB" => lrb::read(&file_bytes).map_err(|e| format!("Failed to parse: {}", e))?,
        "TRK" => trk::read(&file_bytes).map_err(|e| format!("Failed to parse: {}", e))?,
        "SOL" => sol::read(&file_bytes, Some(sol_index))
            .map_err(|e| format!("Failed to parse: {}", e))?,
        _ => Err(format!("Unsupported 'from' format"))?,
    };

    match to_format.to_uppercase().as_str() {
        "JSON" => {
            let json_str = trackjson::write(&internal_format)
                .map_err(|e| format!("Failed to write: {}", e))?;
            Ok(json_str.into_bytes())
        }
        "LRB" => lrb::write(&internal_format).map_err(|e| format!("Failed to write: {}", e)),
        "SOL" => sol::write(&internal_format).map_err(|e| format!("Failed to write: {}", e)),
        _ => Err(format!("Unsupported 'to' format")),
    }
}

#[tauri::command]
fn get_max_sol_index(file_bytes: Vec<u8>) -> u32 {
    return sol::get_track_count(&file_bytes) - 1;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_files, get_max_sol_index])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
