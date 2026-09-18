use std::collections::HashMap;
use std::path::PathBuf;

use dircpy::CopyBuilder;
use tauri::{AppHandle, Manager};
#[cfg(desktop)]
use tauri_plugin_dialog::DialogExt;

/// Resolves where a new project should be created.
///
/// On desktop, this asks the user to pick a folder via a native dialog,
/// since `tauri-plugin-dialog`'s folder picker is desktop-only (iOS/Android
/// don't expose an arbitrary "browse the filesystem" affordance the same
/// way). On mobile, there's nothing to pick from outside the app's sandbox
/// anyway, so we just use the app's Documents directory automatically.
#[cfg(desktop)]
fn resolve_project_location(app: &AppHandle) -> Result<PathBuf, String> {
    let file_path = app
        .dialog()
        .file()
        .set_title("Project Location")
        .blocking_pick_folder();

    let file_path = file_path.ok_or_else(|| "No folder selected".to_string())?;

    file_path
        .as_path()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "Selected folder path is not a local path".to_string())
}

#[cfg(not(desktop))]
fn resolve_project_location(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .document_dir()
        .map_err(|e| format!("Failed to resolve documents directory: {}", e))
}

#[tauri::command]
pub async fn create_template(
    app: AppHandle,
    template: String,
    name: String,
    parameters: HashMap<String, String>,
) -> Result<String, String> {
    let template_dir = app
        .path()
        .resolve("templates", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve template directory: {}", e))?;
    let template_path = template_dir.join(&template);
    if !template_path.exists() {
        return Err(format!("Template '{}' does not exist", template));
    }

    let project_location = resolve_project_location(&app)?;
    let target_path = project_location.join(&name);
    if target_path.exists() {
        return Err(format!(
            "Target path '{}' already exists",
            target_path.display()
        ));
    }
    std::fs::create_dir_all(&target_path)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;

    if !template_path.is_dir() {
        return Err(format!(
            "Template path '{}' is not a directory",
            template_path.display()
        ));
    }

    CopyBuilder::new(&template_path, &target_path)
        .run()
        .map_err(|e| format!("Failed to copy template: {}", e))?;

    let walker = walkdir::WalkDir::new(&target_path)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        let path = entry.path();
        if path.is_file() {
            let mut content = std::fs::read(path)
                .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;

            let mut filename = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or("".to_string());

            for (key, value) in &parameters {
                if filename.contains(&format!("{{{{{}}}}}", key)) {
                    filename = filename.replace(&format!("{{{{{}}}}}", key), value);
                    let new_path = path.with_file_name(&filename);
                    std::fs::rename(path, &new_path).map_err(|e| {
                        format!("Failed to rename file '{}': {}", path.display(), e)
                    })?;
                }
            }

            if let Ok(s) = String::from_utf8(content) {
                let mut replaced = s;
                for (key, value) in &parameters {
                    replaced = replaced.replace(&format!("{{{{{}}}}}", key), value);
                }
                content = replaced.into_bytes();
            } else {
                continue;
            }

            let final_path = path.with_file_name(&filename);
            std::fs::write(&final_path, content)
                .map_err(|e| format!("Failed to write file '{}': {}", path.display(), e))?;
        }
    }

    Ok(target_path.to_string_lossy().to_string())
}

