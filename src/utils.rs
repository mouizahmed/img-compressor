use std::path::{Path, PathBuf};

pub fn default_output_file(input_file: &str, gif: bool) -> Result<String, String> {
    let input_path = Path::new(input_file);
    let stem = input_path.file_stem().ok_or("Invalid input path")?;
    let extension = input_path.extension().unwrap_or_default().to_str().unwrap_or("png");

    let out_dir = Path::new("images");
    if !out_dir.exists() {
        
    }

  
}