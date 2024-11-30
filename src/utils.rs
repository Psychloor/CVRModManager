use crate::{api::api_error::ApiError, config};
use std::path::Path;
use tokio::fs::File;

pub(crate) async fn create_file_with_directories(path: &Path) -> Result<File, ApiError> {
    // Ensure the parent directory exists
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Create the file
    let file = File::create(path).await?;

    Ok(file)
}

pub(crate) async fn get_all_files_in_directory(
    path: &Path,
    filter: &str,
) -> Result<Vec<String>, ApiError> {
    let mut entries = tokio::fs::read_dir(path).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();

            if filter.is_empty() || file_name.contains(filter) {
                files.push(file_name);
            }
        }
    }

    Ok(files)
}

pub fn is_melon_loader_installed() -> bool {
    let chillout_folder_path = Path::new(config::CONFIGURATION_INSTANCE.chillout_folder());

    let version_dll = chillout_folder_path
        .join("version.dll")
        .try_exists()
        .unwrap_or(false);

    let bootstrap_dll = chillout_folder_path
        .join("MelonLoader")
        .join("Dependencies")
        .join("Bootstrap.dll")
        .try_exists()
        .unwrap_or(false);

    version_dll && bootstrap_dll
}

/// Removes the `MelonLoader` files and directories.
///
/// # Errors
///
/// This function will return an error if it fails to remove any of the files or directories.
pub fn remove_melon_loader() -> Result<(), String> {
    let chillout_folder_path = Path::new(config::CONFIGURATION_INSTANCE.chillout_folder());

    let version_dll = chillout_folder_path.join("version.dll");
    let dobby_dll = chillout_folder_path.join("dobby.dll");
    let melon_loader = chillout_folder_path.join("MelonLoader");

    if version_dll.try_exists().unwrap_or(false) {
        std::fs::remove_file(&version_dll)
            .map_err(|e| format!("Failed to remove version.dll: {e}"))?;
    }

    if dobby_dll.try_exists().unwrap_or(false) {
        std::fs::remove_file(&dobby_dll).map_err(|e| format!("Failed to remove dobby.dll: {e}"))?;
    }

    if melon_loader.try_exists().unwrap_or(false) {
        std::fs::remove_dir_all(&melon_loader)
            .map_err(|e| format!("Failed to remove MelonLoader folder: {e}"))?;
    }

    Ok(())
}
