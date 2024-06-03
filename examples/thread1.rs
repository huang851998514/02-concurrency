use std::{sync::mpsc, thread, time::Duration};

use anyhow::{Ok, Result};

const NUM_THREADS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let cusomter_handler = thread::spawn(move || {
        for msg in rx {
            println!("customer:{:?}", msg);
        }
        println!("消费者退出");
        40
    });
    let result_num = cusomter_handler.join().unwrap();
    println!("消费者线程返回数字：{}", result_num);
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 5;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 5 == 0 {
            println!("生产者 {} 退出", idx);
            break;
        }
    }
    Ok(())
}
