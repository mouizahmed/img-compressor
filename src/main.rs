mod cli;
mod image_processor;
mod prefix_sum_matrix;
mod quad_tree;
mod utils;

use cli::parse_args;
use gif::{Encoder as GifEncoder, Frame, Repeat};
use image_processor::ImageData;
use quad_tree::QuadTree;
use std::fs::File;
use std::io::BufWriter;
use utils::{
    default_output_file, ensure_valid_output_file, hex_to_rgb, print_failure, print_progress,
    print_step, print_success,
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
    print_step("Loading image data");
    let data = match ImageData::from_path(&args.input_file) {
        Ok(data) => {
            print_success();
            data
        }
        Err(e) => {
            print_failure();
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
            println!(
                "Generating animated GIF with {} total iterations...",
                args.iterations
            );
            let mut frames = Vec::new();

            // Initial frame
            print_step("Rendering initial frame");
            let buf = quad_tree.render_rgba(outline_rgb);
            let width = buf.width() as u16;
            let height = buf.height() as u16;
            let mut raw_data = buf.into_raw();
            frames.push(Frame::from_rgba_speed(width, height, &mut raw_data, 10));
            print_success();

            // Process iterations
            for i in 1..=args.iterations {
                print_progress(i as usize, args.iterations as usize, "Processing");

                if let Err(err) = quad_tree.split_next() {
                    println!("\n{}", err);
                    std::process::exit(1);
                }

                if i % delta == 0 {
                    let buf = quad_tree.render_rgba(outline_rgb);
                    let mut raw_data = buf.into_raw();
                    frames.push(Frame::from_rgba_speed(width, height, &mut raw_data, 10));
                }
            }

            print_step("Encoding GIF");
            let file = match File::create(&output_file) {
                Ok(file) => file,
                Err(_) => {
                    print_failure();
                    eprintln!("Unable to create output file");
                    std::process::exit(1);
                }
            };
            let writer = BufWriter::new(file);
            let mut encoder =
                GifEncoder::new(writer, frames[0].width, frames[0].height, &[]).unwrap();
            encoder.set_repeat(Repeat::Infinite).unwrap();

            for (i, frame) in frames.into_iter().enumerate() {
                if encoder.write_frame(&frame).is_err() {
                    print_failure();
                    eprintln!("Error encoding gif frame {}", i);
                    std::process::exit(1);
                }
            }
            print_success();
        }
        None => {
            println!("Processing {} iterations...", args.iterations);
            for i in 1..=args.iterations {
                print_progress(i as usize, args.iterations as usize, "Processing");

                if let Err(err) = quad_tree.split_next() {
                    println!("\n{}", err);
                    std::process::exit(1);
                }
            }

            print_step("Saving image");
            if let Err(err) = quad_tree.render_rgb(outline_rgb).save(&output_file) {
                print_failure();
                eprintln!("Error saving image: {}", err);
                std::process::exit(1);
            }
            print_success();
        }
    }

    println!("Compression complete! Output saved to: {}", output_file);
}
