mod cli;
mod utils;
mod image_processor;
mod prefix_sum_matrix;

use cli::parse_args;
use image_processor::ImageData;
use utils::{default_output_file, ensure_valid_output_file, hex_to_rgb};

fn main() {
    let args = parse_args();
    
    println!("Image Compressor");
    println!("Input file: {}", args.input_file);

    // Convert outline hex to RGB if provided
    let outline_rgb = if let Some(outline_hex) = &args.outline {
        match hex_to_rgb(outline_hex) {
            Ok(rgb) => {
                println!("Outline color: {} -> RGB({}, {}, {})", outline_hex, rgb.r, rgb.g, rgb.b);
                Some(rgb)
            }
            Err(e) => {
                eprintln!("Error parsing outline color '{}': {}", outline_hex, e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };
    
    // Handle output file validation
    let output_file = if let Some(user_output) = &args.output_file {
        match ensure_valid_output_file(user_output, &args.input_file, args.gif_delta.is_some()) {
            Ok(validated_path) => {
                if validated_path != *user_output {
                    println!("Output file corrected: {} -> {}", user_output, validated_path);
                }
                validated_path
            }
            Err(e) => {
                eprintln!("Error validating output file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match default_output_file(&args.input_file, args.gif_delta.is_some()) {
            Ok(default_path) => default_path,
            Err(e) => {
                eprintln!("Error generating default output file: {}", e);
                std::process::exit(1);
            }
        }
    };

    println!("Output file: {}", output_file);

    let data = match ImageData::from_path(&args.input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error processing image: {}", e);
            std::process::exit(1);
        }
    };
    
    // TODO: Add image compression logic here
    println!("Processing image...");
}
