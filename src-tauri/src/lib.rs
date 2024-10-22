use serde_json::Value;
use std::path::PathBuf;
use tauri::{generate_handler, Builder, Manager, Window};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

#[tauri::command]
async fn get_ffmpeg_path() -> Result<String, String> {
    // Attempt to resolve the app data directory
    match std::env::current_exe() {
        Ok(dir) => Ok(dir.display().to_string()), // Return the directory as a String if found
        Err(e) => Err(format!("Couldn't resolve app data dir: {}", e)), // Return an error if not found
    }
}

#[tauri::command]
async fn get_video(window: Window, url: String, path: String) -> Result<bool, String> {
    // Check if the URL is empty
    if url.is_empty() {
        return Err("URL is empty.".to_string()); // Return an error if the URL is empty
    }

    // Initialize the sidecar command for yt-dlp
    let sidecar_command = window
        .app_handle()
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to initialize sidecar: {}", e))?;

    let ffmpeg_path = get_ffmpeg_path().await?;
    let mut pathf = PathBuf::from(ffmpeg_path);
    pathf.pop(); // Remove the original executable name
    pathf.push("ffmpeg"); // Add "ffmpeg" as the new binary name

    pathf.to_string_lossy().to_string(); // Convert to String
    let (mut rx, _child) = sidecar_command
        .arg(&url)
        .arg("-o")
        .arg(format!("{}/%(title)s", path))
        .arg("--ffmpeg-location")
        .arg(pathf)
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line_bytes) => {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("yt-dlp output: {}", line); // Optional: Debugging output
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

    Ok(true)
}

#[tauri::command]
async fn get_title(window: Window, url: String) -> Result<(String, String), String> {
    if url.is_empty() {
        return Err("URL is empty.".to_string());
    }
    // Initialize the sidecar command for yt-dlp
    let sidecar_command = window
        .app_handle()
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Failed to initialize sidecar: {}", e))?;
    // let which_command = window
    //     .app_handle()
    //     .shell()
    //     .sidecar("which")
    //     .map_err(|e| format!("Failed to initialize sidecar: {}", e))?;

    // Build output path string

    // let ffmpeg_output = which_command
    //     .arg("ffmpeg")
    //     .output()
    //     .await // Await the Future to get the result
    //     .map_err(|e| format!("Failed to find ffmpeg: {}", e))?;

    // // Check if ffmpeg was found
    // if !ffmpeg_output.status.success() {
    //     return Err("ffmpeg not found.".to_string());
    // }

    // let ffmpeg_path = String::from_utf8_lossy(&ffmpeg_output.stdout)
    //     .trim()
    //     .to_string();
    // print!("ffmpeg path: {}", ffmpeg_path);

    // // Spawn the yt-dlp sidecar with the provided URL
    let ffmpeg_path = get_ffmpeg_path().await?;
    let mut path = PathBuf::from(ffmpeg_path);
    path.pop(); // Remove the original executable name
    path.push("ffmpeg"); // Add "ffmpeg" as the new binary name

    path.to_string_lossy().to_string(); // Convert to String

    let (mut rx, _child) = sidecar_command
        .arg("-j") // Request JSON output
        .arg(&url)
        .arg("--ffmpeg-location")
        .arg(path)
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
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(generate_handler![get_title, get_video, get_ffmpeg_path]) // Register the get_title command
        .run(tauri::generate_context!()) // Run the Tauri app
        .expect("Error while running Tauri application");
}
