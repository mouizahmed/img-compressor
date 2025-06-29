use crate::image_processor::RGB;
use std::path::Path;

pub fn hex_to_rgb(hex: &str) -> Result<RGB<u8>, String> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 { 
        return Err("Invalid hex color".into());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;

    Ok(RGB::new(r, g, b))
}

pub fn ensure_valid_output_file(output_file: &str, input_file: &str, gif: bool) -> Result<String, String> {
    let output_path = Path::new(output_file);
    let input_path = Path::new(input_file);
    
    let input_extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| format!("Input file '{}' has no valid extension", input_file))?;
    
    let output_stem = output_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Invalid output file path: '{}'", output_file))?;
    
    let parent_dir = output_path.parent().unwrap_or_else(|| Path::new("."));
    
    let target_extension = if gif {
        "gif"
    } else {
        &input_extension.to_lowercase()
    };
    
    let mut corrected_path = parent_dir.to_path_buf();
    corrected_path.push(output_stem);
    corrected_path.set_extension(target_extension);
    
    corrected_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to convert path to string".to_string())
}

pub fn default_output_file(input_file: &str, gif: bool) -> Result<String, String> {
    let input_path = Path::new(input_file);
    
    let stem = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Input file '{}' has no valid filename", input_file))?;
    
    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| format!("Input file '{}' has no valid extension", input_file))?;

    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let mut out_path = parent_dir.to_path_buf();
    out_path.push(format!("{}-compressed", stem));
    
    if gif {
        out_path.set_extension("gif");
    } else {
        out_path.set_extension(extension.to_lowercase());
    }
    
    out_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to convert path to string".to_string())
}