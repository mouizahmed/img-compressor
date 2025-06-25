mod cli;

use cli::parse_args;

fn main() {
    let args = parse_args();
    
    println!("Image Compressor");
    println!("Input file: {}", args.input_file);
    
    if let Some(output) = &args.output_file {
        println!("Output file: {}", output);
    } else {
        println!("Output file: (will use default name)");
    }
    
    println!("Iterations: {}", args.iterations);
    
    if let Some(outline) = &args.outline {
        println!("Outline color: {}", outline);
    }
    
    if let Some(delta) = args.gif_delta {
        println!("GIF delta: {}", delta);
    }
    
    // TODO: Add your image compression logic here
    println!("Processing image...");
}
