use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const RAW_EXTENSIONS: [&str; 7] = ["cr2", "nef", "arw", "rw2", "dng", "raw", "orf"];

pub fn convert_raw_to_jpg(input_dir: &str, output_dir: Option<&str>) -> Result<(), String> {
    let input_path = Path::new(input_dir);
    if !input_path.exists() || !input_path.is_dir() {
        return Err("입력 디렉토리가 유효하지 않습니다".to_string());
    }

    let output_path = output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| input_path.join("jpg"));

    fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(input_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        if let Some(ext) = ext {
            if RAW_EXTENSIONS.contains(&ext.as_str()) {
                let filename = path.file_stem().unwrap().to_string_lossy();
                let output_file = output_path.join(format!("{filename}.jpg"));

                let status = Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "dcraw -c \"{}\" | convert - \"{}\"",
                        path.display(),
                        output_file.display()
                    ))
                    .status()
                    .map_err(|e| e.to_string())?;

                if !status.success() {
                    return Err(format!("{} 변환 실패", path.display()));
                }
            }
        }
    }

    Ok(())
}
