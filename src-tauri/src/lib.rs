use std::fs;
use std::io::Read;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;


#[tauri::command]
fn getIconData(path: &str) -> Result<Vec<serde_json::Value>, String> {
    let perks_dir = std::env::current_dir().map_err(|e| e.to_string())?.join(path);
    let mut file_data_list = Vec::new();
    if perks_dir.exists() && perks_dir.is_dir() {
        for entry in fs::read_dir(perks_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_type().map_err(|e| e.to_string())?.is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".png") {
                        let mut file = fs::File::open(entry.path()).map_err(|e| e.to_string())?;
                        let mut buffer = Vec::new();
                        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                        let base64_data = STANDARD.encode(&buffer);
                        let mut fileName = file_name.to_string();
                        fileName.truncate(fileName.len().saturating_sub(4));
                        file_data_list.push(serde_json::json!({
                            "Name": fileName,
                            "Data": base64_data
                        }));
                    }
                }
            }
        }
    }

    Ok(file_data_list)
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![getIconData])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
