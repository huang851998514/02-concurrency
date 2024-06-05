use std::ops::{Add, AddAssign, Mul};

use anyhow::Result;

use crate::Vector;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Martix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Martix<T> {
    pub fn new(data: Vec<T>, row: usize, col: usize) -> Self {
        Self { data, row, col }
    }
}

#[allow(dead_code)]
fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Invalid vector size, a.len != b.len"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}

pub fn multiply<T>(a: &Martix<T>, b: &Martix<T>) -> Result<Martix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    let mut data = vec![T::default(); a.row * b.col];
    if a.col != b.row {
        return Err(anyhow::anyhow!("Invalid matrix size, a.col != b.row"));
    }
    // 矩阵乘法
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    let result = Martix::new(data, a.row, b.col);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Martix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Martix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let result = multiply(&a, &b).unwrap();
        assert_eq!(result.data, vec![22, 28, 49, 64]);
    }
}
