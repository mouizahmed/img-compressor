# img-compressor

Fast and efficient quad-tree based image compression CLI made in Rust.

## Overview

`img-compressor` uses a quad-tree algorithm to compress images by iteratively subdividing regions based on color variance. The tool can generate both static compressed images and animated GIFs showing the compression process.

## Features

- 🚀 **Fast**: Built in Rust with optimized algorithms
- 🎯 **Configurable**: Control compression level with iteration count
- 🎨 **Visual**: Optional outline rendering to show quad-tree structure  
- 📹 **Animated**: Generate GIFs showing the compression process
- 🔧 **CLI**: Simple command-line interface

## Examples

|original | 100 Iterations           | 1,000 Iterations          | 20,000 Iterations          |
| ------------------------ | ------------------------ | ------------------------- | -------------------------- |
| ![](images/example1/cat.jpg) | ![](images/example1/cat-compressed-100.jpg)  | ![](images/example1/cat-compressed-1000.jpg)  | ![](images/example1/cat-compressed-20000.jpg)  |
| 3.1 MB | 524 KB | 712 KB | 1.1 MB |

## Installation

### From Source
```bash
git clone https://github.com/mouizahmed/img-compressor.git
cd img-compressor
cargo build --release
```

The binary will be available at `target/release/img-compressor` (or `img-compressor.exe` on Windows).

## Command Line Options

```
USAGE:
    img-compressor [OPTIONS] --iterations <N> <FILE>

ARGUMENTS:
    <FILE>    Input image file

OPTIONS:
    --iterations <N>        Number of refinement iterations
    --output-file <FILE>    Output file path (optional)
    --outline <HEX>         Outline color in hex format (e.g. #000000) (optional)
    --gif-delta <N>         Save algorithm process to GIF, frame every N iterations
    -h, --help              Print help information
```

## Usage

### Basic Compression
```bash
# Compress an image with 50 iterations
./img-compressor input.jpg --iterations 50

# Output: input-compressed.jpg
```

### Custom Output File
```bash
# Specify custom output filename
./img-compressor input.jpg --iterations 100 --output-file result.jpg
```

### Add Outline
```bash
# Add black outline to show quad-tree structure
./img-compressor input.jpg --iterations 75 --outline "#000000"

# Add red outline
./img-compressor input.jpg --iterations 75 --outline "#FF0000"
```

### Generate Animated GIF
```bash
# Create GIF with frame every 5 iterations
./img-compressor input.jpg --iterations 50 --gif-delta 5

# Create smooth animation with frame every iteration
./img-compressor input.jpg --iterations 20 --gif-delta 1 --output-file smooth.gif

# GIF with outline
./img-compressor input.jpg --iterations 30 --gif-delta 3 --outline "#FFFFFF" --output-file outlined.gif
```

## Algorithm Details

The quad-tree compression algorithm works by:

1. **Initial State**: Start with the entire image as a single region
2. **Variance Calculation**: Calculate color variance for each region using prefix sum matrices for O(1) queries
3. **Priority Selection**: Use a max-heap to always split the region with highest variance
4. **Subdivision**: Split selected regions into 4 quadrants
5. **Iteration**: Repeat until desired number of iterations

### Key Features:
- **O(1) variance queries** using prefix sum matrices
- **Optimal splitting** using priority queue (max-heap)
- **Efficient rendering** with breadth-first traversal
- **Memory safe** implementation in Rust

### Time Complexity Analysis

**Overall Time Complexity: `O(W × H + k × log k)`**

Where:
- `W × H` = image dimensions (width × height)
- `k` = number of iterations

#### Breakdown by Phase:

**1. Image Loading & Preprocessing: `O(W × H)`**
- Read and convert each pixel: `O(W × H)`
- Build prefix sum matrices: `O(W × H)`
- Build squared prefix sums: `O(W × H)`

**2. Per-Iteration Processing: `O(log k)`**
- Priority queue operations: `O(log k)` for heap operations
- Variance calculations: `O(1)` per query (4 queries per iteration)
- Node splitting and storage: `O(1)`

**3. Final Rendering: `O(W × H)`**
- Traverse quad-tree and render pixels: `O(W × H)`

#### Performance Characteristics:

- **Typical usage**: Dominated by `O(W × H)` - image processing is the bottleneck
- **Very large iteration counts**: Still efficient due to `log k` growth in quad-tree operations
- **Scaling**: Doubling iterations doesn't double the work due to logarithmic growth

#### Comparison to Naive Approach:
- **Naive variance calculation**: `O(k × W × H)` total
- **This implementation**: `O(W × H + k × log k)` total
- **Speedup**: Massive improvement for large k or large images

**Space Complexity: `O(W × H + k)`**
- Image data and prefix sums: `O(W × H)`
- Quad-tree nodes: `O(k)`
- Priority queue: `O(k)`

## Performance Tips

- **Always use release mode**: `cargo run --release`
- **Start small**: Begin with 50-100 iterations, increase gradually
- **Large images**: Use fewer iterations initially to test
- **GIF generation**: Slower than static images, use smaller `--gif-delta` values

## File Format Support

**Input formats:** JPEG, PNG, BMP, TIFF, WebP, and other formats supported by the `image` crate

**Output formats:** 
- Static: JPEG, PNG (determined by input format or --output-file extension)
- Animated: GIF

## License

[LICENSE](LICENSE)

## Acknowledgements

- [QuadTreeImageCompression](https://github.com/Inspiaaa/QuadTreeImageCompression) - Original inspiration
- [Quad tree structures for image compression applications](https://www.sciencedirect.com/science/article/abs/pii/0306457392900636)