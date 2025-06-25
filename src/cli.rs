use clap::Parser;

#[derive(Parser)]
#[command(name = "img-compressor")]
#[command(about = "Compress images with iterative refinement")]
pub struct Args {
    /// Input image file
    #[arg(value_name = "FILE")]
    pub input_file: String,
    
    /// Output file path (optional)
    #[arg(long, value_name = "FILE")]
    pub output_file: Option<String>,
    
    /// Number of refinement iterations
    #[arg(long, value_name = "N")]
    pub iterations: u32,
    
    /// Outline color in hex format (e.g. #000000) (optional)
    #[arg(long, value_name = "HEX")]
    pub outline: Option<String>,
    
    /// Save the algorithm process to a GIF, save the image every N iterations
    #[arg(long, value_name = "N")]
    pub gif_delta: Option<u32>,
}

pub fn parse_args() -> Args {
    Args::parse()
}