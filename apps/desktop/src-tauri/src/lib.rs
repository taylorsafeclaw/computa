use tauri::tray::TrayIconBuilder;

/// Inject the given text at the current cursor position.
///
/// Stub: real injection (enigo / macOS Accessibility) lands in a follow-up plan.
/// For now it validates input and logs, returning Ok so the IPC path is exercisable.
#[tauri::command]
fn inject_text(text: String) -> Result<(), String> {
    if text.is_empty() {
        return Err("inject_text: empty text".into());
    }
    println!("[inject_text] would inject {} chars", text.chars().count());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Computa")
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![inject_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::inject_text;

    #[test]
    fn inject_text_rejects_empty() {
        assert!(inject_text(String::new()).is_err());
    }

    #[test]
    fn inject_text_accepts_nonempty() {
        assert!(inject_text("hello".into()).is_ok());
    }
}
