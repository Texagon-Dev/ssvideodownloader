// use std::ptr::metadata;

use serde_json::Value;
use tauri::{generate_handler, Builder, Manager, Window};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

#[tauri::command]
async fn get_title(window: Window, url: String) -> Result<(String, String), String> {
    // Initialize the sidecar command for yt-dlp
    let sidecar_command = window
        .app_handle()
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to initialize sidecar: {}", e))?;

    // Spawn the yt-dlp sidecar with the provided URL
    let (mut rx, _child) = sidecar_command
        .arg("-j") // Request JSON output
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    let mut title = String::new();
    let mut thumbnail = String::new();

    // Read stdout asynchronously from the sidecar
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line_bytes) => {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("yt-dlp output: {}", line); // Optional: Debugging output

                // Parse the JSON from yt-dlp output
                let metadata: Value = serde_json::from_str(&line)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;

                // Extract title if available
                if let Some(t) = metadata["title"].as_str() {
                    title = t.to_string();
                }

                // Check for the thumbnail field
                if let Some(thumbnail_value) = metadata.get("thumbnail") {
                    // If it's an array
                    if let Some(thumbnails) = thumbnail_value.as_array() {
                        if let Some(last_thumbnail) = thumbnails.last() {
                            if let Some(t) = last_thumbnail["url"].as_str() {
                                thumbnail = t.to_string();
                            } else {
                                thumbnail = "".to_string();
                            }
                        }
                    } else if let Some(t) = thumbnail_value.as_str() {
                        // If it's a single URL
                        thumbnail = t.to_string();
                    } else {
                        return Err("Thumbnail field is not a valid type.".to_string());
                    }
                }
            }
            CommandEvent::Stderr(err_bytes) => {
                let err = String::from_utf8_lossy(&err_bytes);
                eprintln!("yt-dlp error: {}", err); // Optional: Debugging errors
                return Err(format!("yt-dlp error: {}", err));
            }
            CommandEvent::Error(error) => {
                return Err(format!("Process error: {}", error));
            }
            _ => {} // Ignore other events
        }
    }

    // Return the extracted title or a fallback message
    if title.is_empty() {
        Err("Failed to retrieve the video title.".to_string())
    } else {
        Ok((title, thumbnail))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init()) // Correct initialization
        .invoke_handler(generate_handler![get_title]) // Register the get_title command
        .run(tauri::generate_context!()) // Run the Tauri app
        .expect("Error while running Tauri application");
}
