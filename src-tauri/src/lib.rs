use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use rdev::{simulate, Button, EventType};
use std::thread;
use std::time::Duration;

// Global state for autoclicker
static IS_CLICKING: AtomicBool = AtomicBool::new(false);
static CURRENT_BUTTON: Mutex<String> = Mutex::new(String::new());

#[tauri::command]
fn start_clicking(interval_ms: u64, button: String) -> Result<String, String> {
    if IS_CLICKING.load(Ordering::SeqCst) {
        return Err("Autoclicker is already running".to_string());
    }

    IS_CLICKING.store(true, Ordering::SeqCst);
    
    // Store the button type
    if let Ok(mut current_btn) = CURRENT_BUTTON.lock() {
        *current_btn = button.clone();
    }

    thread::spawn(move || {
        let mouse_button = match button.as_str() {
            "Middle" => Button::Middle,
            "Right" => Button::Right,
            _ => Button::Left,
        };

        while IS_CLICKING.load(Ordering::SeqCst) {
            if let Err(e) = simulate(&EventType::ButtonPress(mouse_button)) {
                eprintln!("Failed to press button: {:?}", e);
            }
            thread::sleep(Duration::from_millis(interval_ms / 2));

            if let Err(e) = simulate(&EventType::ButtonRelease(mouse_button)) {
                eprintln!("Failed to release button: {:?}", e);
            }
            thread::sleep(Duration::from_millis(interval_ms / 2));
        }
    });

    Ok("Autoclicker started".to_string())
}

#[tauri::command]
fn stop_clicking() -> String {
    IS_CLICKING.store(false, Ordering::SeqCst);
    "Autoclicker stopped".to_string()
}

#[tauri::command]
fn is_clicking() -> bool {
    IS_CLICKING.load(Ordering::SeqCst)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![start_clicking, stop_clicking, is_clicking])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
