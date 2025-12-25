use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use rdev::{simulate, Button, EventType};
use std::thread;
use std::time::Duration;

// Security constants for input validation
const MIN_INTERVAL_MS: u64 = 10;
const MAX_INTERVAL_MS: u64 = 10000;

// Valid button types
const VALID_BUTTONS: [&str; 3] = ["Left", "Middle", "Right"];

// Global state for autoclicker
static IS_CLICKING: AtomicBool = AtomicBool::new(false);
static CURRENT_BUTTON: Mutex<Option<String>> = Mutex::new(None);
static CLICK_COUNT: AtomicU64 = AtomicU64::new(0);

/// Validates and sanitizes the button input
fn validate_button(button: &str) -> Result<Button, String> {
    match button {
        "Left" => Ok(Button::Left),
        "Middle" => Ok(Button::Middle),
        "Right" => Ok(Button::Right),
        _ => Err(format!(
            "Invalid button type '{}'. Must be one of: {:?}",
            button, VALID_BUTTONS
        )),
    }
}

/// Validates the interval is within acceptable bounds
fn validate_interval(interval_ms: u64) -> Result<u64, String> {
    if interval_ms < MIN_INTERVAL_MS {
        return Err(format!(
            "Interval too small: {}ms. Minimum is {}ms",
            interval_ms, MIN_INTERVAL_MS
        ));
    }
    if interval_ms > MAX_INTERVAL_MS {
        return Err(format!(
            "Interval too large: {}ms. Maximum is {}ms",
            interval_ms, MAX_INTERVAL_MS
        ));
    }
    Ok(interval_ms)
}

#[tauri::command]
fn start_clicking(interval_ms: u64, button: String) -> Result<String, String> {
    // Validate inputs before proceeding
    let validated_interval = validate_interval(interval_ms)?;
    let mouse_button = validate_button(&button)?;

    // Check if already running (atomic check-and-set)
    if IS_CLICKING.swap(true, Ordering::SeqCst) {
        return Err("Autoclicker is already running".to_string());
    }
    
    // Reset click counter
    CLICK_COUNT.store(0, Ordering::SeqCst);
    
    // Store the button type with proper mutex handling
    match CURRENT_BUTTON.lock() {
        Ok(mut current_btn) => {
            *current_btn = Some(button.clone());
        }
        Err(poisoned) => {
            // Recover from poisoned mutex
            let mut current_btn = poisoned.into_inner();
            *current_btn = Some(button.clone());
        }
    }

    // Calculate sleep durations (ensure minimum 1ms for each phase)
    let press_duration = Duration::from_millis((validated_interval / 2).max(1));
    let release_duration = Duration::from_millis((validated_interval - validated_interval / 2).max(1));

    thread::spawn(move || {
        while IS_CLICKING.load(Ordering::SeqCst) {
            if let Err(e) = simulate(&EventType::ButtonPress(mouse_button)) {
                eprintln!("Failed to press button: {:?}", e);
                // Continue running despite errors
            }
            thread::sleep(press_duration);

            if let Err(e) = simulate(&EventType::ButtonRelease(mouse_button)) {
                eprintln!("Failed to release button: {:?}", e);
            }
            
            // Increment click counter
            CLICK_COUNT.fetch_add(1, Ordering::SeqCst);
            
            thread::sleep(release_duration);
        }
    });

    Ok(format!("Autoclicker started: {} button at {}ms interval", button, validated_interval))
}

#[tauri::command]
fn stop_clicking() -> Result<String, String> {
    // Use swap to atomically get previous state and set to false
    let was_running = IS_CLICKING.swap(false, Ordering::SeqCst);
    
    // Clear the stored button type
    match CURRENT_BUTTON.lock() {
        Ok(mut current_btn) => {
            *current_btn = None;
        }
        Err(poisoned) => {
            // Recover from poisoned mutex
            let mut current_btn = poisoned.into_inner();
            *current_btn = None;
        }
    }
    
    if was_running {
        Ok("Autoclicker stopped".to_string())
    } else {
        Ok("Autoclicker was not running".to_string())
    }
}

#[tauri::command]
fn is_clicking() -> bool {
    IS_CLICKING.load(Ordering::SeqCst)
}

/// Get the current click count
#[tauri::command]
fn get_click_count() -> u64 {
    CLICK_COUNT.load(Ordering::SeqCst)
}

/// Get the current configuration status (for debugging)
#[tauri::command]
fn get_status() -> Result<String, String> {
    let is_running = IS_CLICKING.load(Ordering::SeqCst);
    let button = match CURRENT_BUTTON.lock() {
        Ok(guard) => guard.clone().unwrap_or_else(|| "None".to_string()),
        Err(poisoned) => poisoned.into_inner().clone().unwrap_or_else(|| "None".to_string()),
    };
    
    Ok(format!(
        "Running: {}, Button: {}",
        is_running, button
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![start_clicking, stop_clicking, is_clicking, get_click_count, get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
