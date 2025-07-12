mod cli;
mod image_processor;
mod prefix_sum_matrix;
mod quad_tree;
mod utils;

use cli::parse_args;
use quad_tree::QuadTree;
use utils::{
    default_output_file, ensure_valid_output_file, hex_to_rgb, load_image_data, print_step,
    print_success, process_gif_compression, process_static_compression,
};

fn main() {
    let args = parse_args();

    println!("Image Compressor");
    println!("Input file: {}", args.input_file);

    // Convert outline hex to RGB if provided
    let outline_rgb = if let Some(outline_hex) = &args.outline {
        match hex_to_rgb(outline_hex) {
            Ok(rgb) => {
                println!(
                    "Outline color: {} -> RGB({}, {}, {})",
                    outline_hex, rgb.r, rgb.g, rgb.b
                );
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
                    println!(
                        "Output file corrected: {} -> {}",
                        user_output, validated_path
                    );
                }
                validated_path
            }
            Err(e) => {
                eprintln!("Error validating output file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match default_output_file(
            &args.input_file,
            args.gif_delta.is_some(),
            args.iterations,
            args.outline.is_some(),
            args.gif_delta,
        ) {
            Ok(default_path) => default_path,
            Err(e) => {
                eprintln!("Error generating default output file: {}", e);
                std::process::exit(1);
            }
        }
    };

    println!("Output file: {}", output_file);
    println!();

    // Load image data
    let data = match load_image_data(&args.input_file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error processing image: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize quad tree
    print_step("Initializing quad tree");
    let mut quad_tree = QuadTree::new(data);
    print_success();

    // Process based on whether GIF output is requested
    match args.gif_delta {
        Some(delta) => {
            if let Err(e) = process_gif_compression(
                &mut quad_tree,
                args.iterations,
                delta,
                outline_rgb,
                &output_file,
            ) {
                eprintln!("Error during GIF compression: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            if let Err(e) = process_static_compression(
                &mut quad_tree,
                args.iterations,
                outline_rgb,
                &output_file,
            ) {
                eprintln!("Error during static compression: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("Compression complete! Output saved to: {}", output_file);
}
