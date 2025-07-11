use crate::image_processor::RGB;
use std::io::{self, Write};
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

pub fn print_progress(current: usize, total: usize, step_name: &str) {
    let percentage = (current as f32 / total as f32 * 100.0) as usize;
    let bar_width = 30;
    let filled = (current as f32 / total as f32 * bar_width as f32) as usize;
    let empty = bar_width - filled;

    print!(
        "\r{}: [{}{}] {}% ({}/{})",
        step_name,
        "█".repeat(filled),
        "░".repeat(empty),
        percentage,
        current,
        total
    );
    io::stdout().flush().unwrap();

    if current == total {
        println!(); // New line when complete
    }
}

pub fn print_step(message: &str) {
    print!("{}...", message);
    io::stdout().flush().unwrap();
}

pub fn print_success() {
    println!(" ✓ Complete");
}

pub fn print_failure() {
    println!(" ✗ Failed");
}

pub fn ensure_valid_output_file(
    output_file: &str,
    input_file: &str,
    gif: bool,
) -> Result<String, String> {
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

pub fn default_output_file(
    input_file: &str,
    gif: bool,
    iterations: u32,
    has_outline: bool,
    gif_delta: Option<u32>,
) -> Result<String, String> {
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

    let filename = match (gif, gif_delta, has_outline) {
        (true, Some(delta), true) => {
            format!("{}-compressed-{}-delta{}-outline", stem, iterations, delta)
        }
        (true, Some(delta), false) => format!("{}-compressed-{}-delta{}", stem, iterations, delta),
        (true, None, true) => format!("{}-compressed-{}-outline", stem, iterations),
        (true, None, false) => format!("{}-compressed-{}", stem, iterations),
        (false, _, true) => format!("{}-compressed-{}-outline", stem, iterations),
        (false, _, false) => format!("{}-compressed-{}", stem, iterations),
    };

    out_path.push(filename);

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
