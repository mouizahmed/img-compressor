use image::Rgb;
use prefix_sum_matrix::PrefixSumMatrix;

pub struct ImageData {
    height: usize,
    width: usize,
    sums: PrefixSumMatrix<Rgb<u64>>,
    square_sums: PrefixSumMatrix<Rgb<u64>>,
}

impl ImageData {
    pub fn new(data: &Vec<Vec<Rgb<u64>>>) -> Result<Self, String> {
        let sums = PrefixSumMatrix::new(data)?;
        let squares = data.into_iter().map(|row| row.into_iter().map(|pixel| pixel.map(|p| p * p)).collect()).collect();
        let square_sums = PrefixSumMatrix::new(&squares)?;

        Ok(Self {
            height: data.len(),
            width: data[0].len(),
            sums,
            square_sums,
        })
    }

    pub fn from_path(path: &str) -> Result<Self, String> {
        let Ok(image) = image::open(path) else {
            return Err(format!("Failed to open image file: {}", path));
        };

        let Ok(image_decoded) = image.decode() else {
            return Err(format!("Failed to decode image: {}", path));
        };

        let Ok(image_rgb) = image_decoded.to_rgb8() else {
            return Err(format!("Failed to convert image to RGB8: {}", path));
        };

        let (width, height) = image_rgb.dimensions();
        let mut data = vec![vec![Rgb([0u64, 0u64, 0u64]); width as usize]; height as usize];

        for (x, y, pixel) in image_rgb.enumerate_pixels() {
            let rgb = Rgb([pixel.0[0] as u64, pixel.0[1] as u64, pixel.0[2] as u64]);
            data[y as usize][x as usize] = rgb;
        }

        Self::new(&data)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn query_sum(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> Rgb<u64> {
        self.sums.query_sum(top_left, bottom_right)
    }

    pub fn query_square_sum(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> Rgb<u64> {
        self.square_sums.query_sum(top_left, bottom_right)
    }

    pub fn query_mean(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> Rgb<u64> {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;
        let area = (x2 - x1 + 1) * (y2 - y1 + 1);

        self.query_sum(top_left, bottom_right).map(|sum| sum / area)
    }

    pub fn query_variance(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> u64 {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;
        let area = (x2 - x1 + 1) * (y2 - y1 + 1);
        
        let square_sum = self.query_square_sum(top_left, bottom_right);
        let mean = self.query_mean(top_left, bottom_right);

        // Calculate variance for each channel: square_sum/area - mean^2
        let variance = square_sum.map(|sq_sum| sq_sum / area).zip(mean.map(|m| m * m)).map(|(sq_mean, mean_sq)| {
            if sq_mean >= mean_sq {
                sq_mean - mean_sq
            } else {
                0 // Handle potential underflow
            }
        });

        // Return sum of RGB components multiplied by area
        (variance.r + variance.g + variance.b) * area
    }
}