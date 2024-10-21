use serde_json::Value;
use std::process::Command;
use tauri::{generate_handler, Builder};
use tauri_plugin_shell::init as shell_init;

#[tauri::command]
fn get_title(url: &str) -> Result<String, String> {
    // Execute the yt-dlp command with JSON output
    let output = Command::new("yt-dlp")
        .arg("-j") // JSON output format to parse video metadata
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?; // Handle errors properly

    // Convert stdout to a UTF-8 string
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("yt-dlp output:\n{}", stdout); // Optional: Debugging

    // Check if stderr contains any errors
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", stderr));
    }

    // Parse the JSON output
    let metadata: Value =
        serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Extract the title from JSON or return a fallback message
    let title = metadata["title"].as_str().unwrap_or("Title not available");

    // Return the video title as a string to be used in the frontend
    Ok(title.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(shell_init()) // Initialize the shell plugin
        .invoke_handler(generate_handler![get_title]) // Register the command
        .run(tauri::generate_context!()) // Run the Tauri app
        .expect("Error while running Tauri application");
}
