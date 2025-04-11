use std::fs;
use std::path::Path;

pub fn organize_by_extension(directory: &str) -> Result<(), String> {
    let dir_path = Path::new(directory);

    if !dir_path.is_dir() {
        return Err(format!("'{}' is not a directory", directory));
    }

    let entries = fs::read_dir(dir_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_lowercase();
            let target_dir = dir_path.join(&ext);

            if !target_dir.exists() {
                fs::create_dir(&target_dir).map_err(|e| e.to_string())?;
            }

            let file_name = path.file_name().ok_or("Invalid file name")?;
            let new_path = target_dir.join(file_name);

            fs::rename(&path, &new_path).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
