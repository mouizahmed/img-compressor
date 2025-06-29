mod cli;
mod utils;

use cli::parse_args;
use utils::{default_output_file, ensure_valid_output_file};

fn main() {
    let args = parse_args();
    
    println!("Image Compressor");
    println!("Input file: {}", args.input_file);
    
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
    println!("Iterations: {}", args.iterations);
    
    if let Some(outline) = &args.outline {
        println!("Outline color: {}", outline);
    }
    
    if let Some(delta) = args.gif_delta {
        println!("GIF delta: {}", delta);
    }
    
    // TODO: Add image compression logic here
    println!("Processing image...");
}
