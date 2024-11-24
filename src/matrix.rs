use core::fmt;
use std::iter::zip;
use std::usize;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<f32>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("\n");
        let max_len = self
            .matrix
            .iter()
            .map(|x| (*x).to_string().len())
            .max()
            .unwrap();

        let rows = self.matrix.as_slice().chunks(self.cols);

        for row in rows {
            result.push_str("[ ");
            row.iter()
                .map(|x| {
                    let str = (*x).to_string();
                    let padded_str = format!("{:>max_len$}", str);

                    return padded_str;
                })
                .for_each(|x| result.push_str(&format!("{} ", x)));
            result.push_str("]\n");
        }

        return write!(f, "{}", result);
    }
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            rows: row,
            cols: col,
            matrix: vec![0.0; row * col],
        }
    }

    pub fn from_vec(v: Vec<Vec<f32>>) -> Self {
        Self {
            rows: v.len(),
            cols: v[0].len(),
            matrix: v.into_iter().flatten().collect(),
        }
    }

    pub fn from_vec_i32(v: Vec<Vec<i32>>) -> Self {
        Self {
            rows: v.len(),
            cols: v[0].len(),
            matrix: v
                .into_iter()
                .map(|x| x.iter().map(|y| *y as f32).collect::<Vec<f32>>())
                .flatten()
                .collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        return self.rows;
    }

    pub fn num_cols(&self) -> usize {
        return self.cols;
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        return self.cols * row + col;
    }

    pub fn at(&self, row: usize, col: usize) -> f32 {
        let val = self.matrix.get(self.index(row, col));

        match val {
            Some(x) => return *x,
            None => return 0.0,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        let index = self.index(row, col);

        if let Some(num) = self.matrix.get_mut(index) {
            *num = value;
        } else {
            panic!("Index out of Matrix bounds");
        }
    }

    pub fn add(&mut self, value: f32) {
        self.matrix.iter_mut().map(|x| *x += value).for_each(|_| {});
    }

    pub fn subtract(&mut self, value: f32) {
        self.matrix.iter_mut().map(|x| *x -= value).for_each(|_| {});
    }

    pub fn multiply(&mut self, value: f32) {
        self.matrix.iter_mut().map(|x| *x *= value).for_each(|_| {});
    }

    pub fn equals(&self, m: &Matrix) -> bool {
        if self.rows != m.rows || self.cols != m.cols {
            return false;
        }

        let result_iter = zip(self.matrix.iter(), m.matrix.iter());

        let results: Vec<_> = result_iter
            .filter(|(x, y)| *x != *y)
            .map(|(x, y)| (*x, *y))
            .collect();

        return results.len() == 0;
    }

    pub fn matrix_add(&self, m: &Matrix) -> Option<Matrix> {
        if self.rows != m.rows || self.cols != m.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols);
        let result_iter = zip(self.matrix.iter(), m.matrix.iter());

        result_iter
            .map(|(x, y)| *x + *y)
            .enumerate()
            .for_each(|(i, z)| {
                if let Some(num) = result.matrix.get_mut(i) {
                    *num = z;
                }
            });

        return Some(result);
    }

    pub fn matrix_multiply(&self, m: &Matrix) -> Option<Matrix> {
        if self.cols != m.num_rows() {
            return None;
        }
        let mut result = Matrix::new(self.rows, m.num_cols());

        let mut i = 0;
        while i < result.num_rows() {
            let mut j = 0;
            while j < result.num_cols() {
                let mut k = 0;
                while k < m.num_rows() {
                    let val = result.at(i, j) + self.at(i, k) * m.at(k, j);
                    result.set(i, j, val);
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        return Some(result);
    }

    pub fn matrix_multiply2(&self, m: &Matrix) -> Option<Matrix> {
        if self.cols != m.num_rows() {
            return None;
        }

        let mut result = Matrix::new(self.rows, m.num_cols());

        let rows = result.num_rows();
        let cols = result.num_cols();
        result.matrix.iter_mut().enumerate().for_each(|(i, num)| {
            let coords = Matrix::get_coords(i, rows, cols);

            match coords {
                None => return (),
                Some((x, y)) => {
                    let row = self
                        .matrix
                        .as_slice()
                        .chunks(self.cols)
                        .nth(x)
                        .unwrap_or(&[0.0]);
                    let col = m
                        .matrix
                        .iter()
                        .enumerate()
                        .filter(|(i, _num)| i % m.cols == y)
                        .map(|(_j, num)| *num)
                        .collect::<Vec<f32>>();

                    *num = Matrix::dot_product(row, col.as_slice());
                }
            };
        });

        return Some(result);
    }

    pub fn get_coords(index: usize, rows: usize, cols: usize) -> Option<(usize, usize)> {
        if rows == 0 {
            return None;
        }
        return Some((index / cols, index % cols));
    }

    pub fn print_matrix(m: Option<Matrix>) {
        match m {
            Some(mat) => println!("{}", mat.to_string()),
            None => println!("{}", String::from("Invalid operation")),
        }
    }

    fn dot_product(row: &[f32], col: &[f32]) -> f32 {
        let result_iter = zip(row, col);

        return result_iter.map(|(x, y)| (*x) * (*y)).sum();
    }
}
