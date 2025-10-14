// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::{fs, io::Write, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use base64::{Engine as _, engine::general_purpose};

const TEMP_SUBDIR: &str = "audio-recordings"; // change this to configure temp subdirectory

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello you, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_audio_file(bytes: Vec<u8>, filename: Option<String>) -> Result<String, String> {
    println!("save_audio_file: received {} bytes, filename={:?}", bytes.len(), filename);
    let mut base: PathBuf = std::env::temp_dir();
    base.push(TEMP_SUBDIR);

    if let Err(e) = fs::create_dir_all(&base) {
        eprintln!("save_audio_file: failed to create temp dir {:?}: {}", base, e);
        return Err(format!("failed to create temp dir: {}", e));
    }

    let generated = || -> String {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        format!("recording-{}.webm", ts)
    }();

    let file_name = filename.unwrap_or(generated);
    let path = base.join(file_name);
    println!("save_audio_file: writing to {:?}", path);

    match fs::File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&bytes) {
                eprintln!("save_audio_file: write error for {:?}: {}", path, e);
                return Err(format!("failed to write file: {}", e));
            }
        }
        Err(e) => {
            eprintln!("save_audio_file: failed to create file {:?}: {}", path, e);
            return Err(format!("failed to create file: {}", e));
        }
    }

    println!("save_audio_file: successfully saved to {:?}", path);
    
    // Return base64 data URL for immediate playback
    let base64_data = general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:audio/webm;base64,{}", base64_data);
    Ok(data_url)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, save_audio_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
