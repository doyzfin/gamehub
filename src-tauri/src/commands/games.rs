use std::process::Command;
use tauri::AppHandle;
use tauri::Emitter;
use std::thread;

#[tauri::command]
pub fn launch_game(app: AppHandle, executable: String, working_directory: Option<String>) -> Result<(), String> {
    // Basic validation
    if executable.is_empty() {
        return Err("Executable path is empty".into());
    }

    thread::spawn(move || {
        let mut cmd = Command::new(&executable);
        
        if let Some(dir) = working_directory {
            cmd.current_dir(dir);
        }
        
        // Inform frontend that game is starting
        let _ = app.emit("game_started", &executable);
        
        match cmd.status() {
            Ok(status) => {
                let _ = app.emit("game_exited", format!("{} exited with status: {}", executable, status));
            },
            Err(e) => {
                let _ = app.emit("game_exited", format!("Failed to launch {}: {}", executable, e));
            }
        }
    });

    Ok(())
}

#[derive(serde::Serialize)]
pub struct ScannedGame {
    name: String,
    executable: String,
    cover: Option<String>,
}

#[tauri::command]
pub fn scan_games() -> Result<Vec<ScannedGame>, String> {
    let mut discovered: Vec<ScannedGame> = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        use std::path::Path;
        
        let paths_to_scan = vec![
            "C:\\Program Files (x86)\\Steam\\steamapps\\common",
            "C:\\Program Files\\Epic Games",
            "C:\\Games"
        ];
        
        for base_path in paths_to_scan {
            let p = Path::new(base_path);
            if p.exists() && p.is_dir() {
                // Read immediate subdirectories (game folders)
                if let Ok(entries) = fs::read_dir(p) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                let game_dir = entry.path();
                                let game_name = game_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
                                
                                // Do a shallow search for the largest .exe or just the first .exe found
                                if let Ok(sub_entries) = fs::read_dir(&game_dir) {
                                    for sub_entry in sub_entries.flatten() {
                                        if let Ok(sub_type) = sub_entry.file_type() {
                                            if sub_type.is_file() {
                                                let sub_path = sub_entry.path();
                                                if let Some(ext) = sub_path.extension() {
                                                    if ext.to_ascii_lowercase() == "exe" {
                                                        // Found an executable, assume it's the game launcher
                                                        discovered.push(ScannedGame {
                                                            name: game_name.clone(),
                                                            executable: sub_path.to_string_lossy().to_string(),
                                                            cover: None,
                                                        });
                                                        break; // Only pick one per game directory for this simple scanner
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(discovered)
}
