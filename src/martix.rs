use std::{
    ops::{Add, AddAssign, Mul},
    thread,
};

use anyhow::{Ok, Result};
use std::sync::mpsc;

use crate::{dot_product, Vector};

const MAX_THREAD_NUM: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Martix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> Martix<T> {
    pub fn new(data: Vec<T>, row: usize, col: usize) -> Self {
        Self { data, row, col }
    }
}

impl <T> Mul for Martix<T>
where T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).unwrap()
    }
}

pub fn multiply<T>(a: &Martix<T>, b: &Martix<T>) -> Result<Martix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    //let mut data = vec![T::default(); a.row * b.col];
    if a.col != b.row {
        return Err(anyhow::anyhow!("Invalid matrix size, a.col != b.row"));
    }

    let senders = (0..MAX_THREAD_NUM)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("send error: {:?}", e);
                    };
                }
                anyhow::Ok(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // 矩阵乘法
    let martix_len = a.row * b.col;
    let mut data = vec![T::default(); martix_len];
    let mut receivers = Vec::with_capacity(martix_len);
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
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, left_vector, right_vector);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % MAX_THREAD_NUM].send(msg) {
                eprintln!("Send error :{:?}", e);
            };
            receivers.push(rx);
            //data[i * b.col + j] = dot_product(left_vector, right_vector)?;
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
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
        let result = a * b;
        assert_eq!(result.data, vec![22, 28, 49, 64]);
    }
}
