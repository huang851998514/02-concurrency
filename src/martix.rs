use std::ops::{Add, AddAssign, Mul};

use anyhow::Result;

use crate::{dot_product, Vector};

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
            let left_vector = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let right_vector = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            data[i * b.col + j] = dot_product(left_vector, right_vector)?;
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
