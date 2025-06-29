use image::Rgb;
use std::path::Path;

pub fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, String> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 { 
        return Err("Invalid hex color".into());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;

    Ok(Rgb([r, g, b]))
}

const VALID_IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "tga", "webp", "pnm"
];

pub fn is_valid_image_extension(extension: &str) -> bool {
    VALID_IMAGE_EXTENSIONS.contains(&extension.to_lowercase().as_str())
}

/// Ensure the output file has a valid image extension
/// If no extension is provided or an invalid extension is provided, 
/// it will use the input file's extension or default to PNG
pub fn ensure_valid_output_file(output_file: &str, input_file: &str, gif: bool) -> Result<String, String> {
    let output_path = Path::new(output_file);
    
    // If GIF mode is enabled, always use .gif extension
    if gif {
        if let Some(stem) = output_path.file_stem() {
            let parent_dir = output_path.parent().unwrap_or_else(|| Path::new(""));
            let gif_path = parent_dir.join(format!("{}.gif", stem.to_string_lossy()));
            return Ok(gif_path.to_string_lossy().to_string());
        }
        return Err("Invalid output file path".into());
    }
    
    // Check if output file has an extension
    if let Some(extension) = output_path.extension() {
        let ext_str = extension.to_str().unwrap_or("").to_lowercase();
        
        // If it's a valid image extension, use it as-is
        if is_valid_image_extension(&ext_str) {
            return Ok(output_file.to_string());
        }
        
        // If it's not a valid image extension, replace it with input file's extension or PNG
        let input_path = Path::new(input_file);
        let fallback_extension = input_path
            .extension()
            .and_then(|ext| ext.to_str())
            .filter(|ext| is_valid_image_extension(ext))
            .unwrap_or("png");
        
        if let Some(stem) = output_path.file_stem() {
            let parent_dir = output_path.parent().unwrap_or_else(|| Path::new(""));
            let corrected_path = parent_dir.join(format!("{}.{}", stem.to_string_lossy(), fallback_extension));
            return Ok(corrected_path.to_string_lossy().to_string());
        }
        return Err("Invalid output file path".into());
    }
    
    // No extension provided, add one based on input file or default to PNG
    let input_path = Path::new(input_file);
    let fallback_extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| is_valid_image_extension(ext))
        .unwrap_or("png");
    
    let corrected_path = Path::new(output_file).with_extension(fallback_extension);
    Ok(corrected_path.to_string_lossy().to_string())
}

pub fn default_output_file(input_file: &str, gif: bool) -> Result<String, String> {
    let input_path = Path::new(input_file);
    let stem = input_path.file_stem().ok_or("Invalid input path")?;
    let extension = input_path.extension().unwrap_or_default().to_str().unwrap_or("png");

    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let out_path = parent_dir.join(format!(
        "{}-compressed.{}",
        stem.to_string_lossy(),
        if gif { "gif" } else { extension }
    ));
    println!("Output file: {}", out_path.to_string_lossy());
    Ok(out_path.to_string_lossy().to_string())
}