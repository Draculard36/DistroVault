use serde_json::Value;
use std::{fs, io::Write, path::PathBuf};
use tauri::Manager;

const MAX_BYTES: usize = 2_000_000;

fn data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| format!("Application data path: {e}"))
}

fn valid_data(value: &Value) -> bool {
    let Some(machines) = value.get("machines").and_then(Value::as_array) else {return false};
    let Some(distros) = value.get("distros").and_then(Value::as_array) else {return false};
    machines.iter().all(|m| m.get("id").and_then(Value::as_str).is_some()
        && m.get("name").and_then(Value::as_str).is_some())
      && distros.iter().all(|d| d.get("id").and_then(Value::as_str).is_some()
        && d.get("name").and_then(Value::as_str).is_some()
        && matches!(d.get("status").and_then(Value::as_str),
          Some("current" | "testing" | "tried" | "wishlist")))
}

#[tauri::command]
fn load_data(app: tauri::AppHandle) -> Result<Option<Value>, String> {
    let path = data_dir(&app)?.join("DistroVault-data.json");
    if !path.exists() { return Ok(None); }
    let bytes = fs::read(&path).map_err(|e| format!("Reading saved data: {e}"))?;
    if bytes.len() > MAX_BYTES {return Err("Data file exceeds 2 MB".into());}
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("Saved data is not valid JSON (file left untouched): {e}"))?;
    if !valid_data(&value) {return Err("Saved data has an invalid structure (file left untouched)".into());}
    Ok(Some(value))
}

#[tauri::command]
fn save_data(app: tauri::AppHandle, payload: Value) -> Result<bool, String> {
    if !valid_data(&payload) {return Err("Invalid DistroVault data: save cancelled".into());}
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_BYTES {return Err("Collection exceeds the 2 MB limit".into());}
    let folder = data_dir(&app)?;
    fs::create_dir_all(&folder).map_err(|e| format!("Creating data folder: {e}"))?;
    let dest = folder.join("DistroVault-data.json");
    let temp = folder.join("DistroVault-data.json.tmp");
    let prev = folder.join("DistroVault-data.previous.json");
    {
        let mut file = fs::File::create(&temp).map_err(|e| format!("Creating temporary save: {e}"))?;
        file.write_all(&bytes).map_err(|e| format!("Writing temporary save: {e}"))?;
        file.sync_all().map_err(|e| format!("Syncing save to disk: {e}"))?;
    }
    if dest.exists() {fs::copy(&dest, &prev).map_err(|e| format!("Backing up previous save: {e}"))?;}
    #[cfg(target_os = "windows")]
    if dest.exists() {
        fs::remove_file(&dest).map_err(|e| format!("Preparing data replacement: {e}"))?;
    }
    fs::rename(&temp, &dest).map_err(|e| format!("Replacing data file: {e}"))?;
    Ok(true)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_data, save_data])
        .run(tauri::generate_context!())
        .expect("error while running DistroVault");
}
