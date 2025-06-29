use std::ops::{Add, Sub};

// Add Zero trait for better semantic meaning
pub trait Zero {
    fn zero() -> Self;
}

pub struct PrefixSumMatrix<T>
where 
    T: Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy,
{
    height: usize,
    width: usize,
    data: Vec<Vec<T>>,
}

impl<T> PrefixSumMatrix<T>
where
    T: Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy,
{
    pub fn new(matrix: &Vec<Vec<T>>) -> Result<Self, String> {
        let height = matrix.len();
        let width = match matrix.first() {
            Some(row) => row.len(),
            None => return Err("Empty matrix".into()),
        };
        
        if width == 0 {
            return Err("Matrix has no columns".into());
        }
        
        let mut data = vec![vec![T::zero(); width + 1]; height + 1];
        for i in 0..height {
            for j in 0..width {
                data[i + 1][j + 1] = data[i][j + 1] + data[i + 1][j] - data[i][j] + matrix[i][j];
            }
        }

        Ok(Self { height, width, data })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row][col]
    }

    pub fn query_sum(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> T {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;
        self.data[x2 + 1][y2 + 1] - self.data[x2 + 1][y1] - self.data[x1][y2 + 1] + self.data[x1][y1]
    }
}