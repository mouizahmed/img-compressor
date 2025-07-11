use std::ops::{Add, Div, Mul, Sub};

use crate::prefix_sum_matrix::{PrefixSumMatrix, Zero};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        RGB { r, g, b }
    }
}

impl<T: Mul<Output = T> + Clone + Copy> RGB<T> {
    fn comp_prod(&self, other: Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl From<RGB<u8>> for RGB<u64> {
    fn from(value: RGB<u8>) -> Self {
        RGB::new(value.r.into(), value.g.into(), value.b.into())
    }
}

impl<T: Add<Output = T>> Add for RGB<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T: Sub<Output = T>> Sub for RGB<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T> Div<T> for RGB<T>
where
    T: Div<T, Output = T> + Clone + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl<T: Default> Default for RGB<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl Zero for RGB<u64> {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }
}

pub struct ImageData {
    height: usize,
    width: usize,
    sums: PrefixSumMatrix<RGB<u64>>,
    square_sums: PrefixSumMatrix<RGB<u64>>,
}

impl ImageData {
    pub fn new(data: &Vec<Vec<RGB<u64>>>) -> Result<Self, String> {
        let sums = PrefixSumMatrix::new(data)?;
        let squares = data
            .iter()
            .map(|row| row.iter().map(|x| x.comp_prod(*x)).collect())
            .collect();
        let square_sums = PrefixSumMatrix::new(&squares)?;
        Ok(Self {
            height: sums.height(),
            width: sums.width(),
            sums,
            square_sums,
        })
    }

    pub fn from_path(path: &str) -> Result<Self, String> {
        let Ok(image) = image::open(path) else {
            return Err(format!("Failed to open image file: {}", path));
        };

        let image_rgb = image.to_rgb8();

        let (width, height) = image_rgb.dimensions();
        let mut data = vec![vec![RGB::zero(); width as usize]; height as usize];

        for (x, y, pixel) in image_rgb.enumerate_pixels() {
            let rgb = RGB::new(pixel.0[0] as u64, pixel.0[1] as u64, pixel.0[2] as u64);
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

    pub fn sum(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> RGB<u64> {
        self.sums.query_sum(top_left, bottom_right)
    }

    pub fn square_sum(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> RGB<u64> {
        self.square_sums.query_sum(top_left, bottom_right)
    }

    pub fn average(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> RGB<u64> {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;
        let area = ((x2 - x1 + 1) * (y2 - y1 + 1)) as u64;
        self.sum(top_left, bottom_right) / area
    }

    pub fn variance(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> u64 {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;
        let area = ((x2 - x1 + 1) * (y2 - y1 + 1)) as u64;

        let mean = self.average(top_left, bottom_right);
        let square_sum = self.square_sum(top_left, bottom_right);

        let mean_squared = mean.comp_prod(mean);
        let square_avg = square_sum / area;

        // Use saturating subtraction to prevent overflow
        let variance_r = square_avg.r.saturating_sub(mean_squared.r);
        let variance_g = square_avg.g.saturating_sub(mean_squared.g);
        let variance_b = square_avg.b.saturating_sub(mean_squared.b);

        (variance_r + variance_g + variance_b) * area
    }
}
