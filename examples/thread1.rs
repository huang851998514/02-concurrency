use anyhow::Ok;

fn main() {
    let handle = std::thread::spawn(|| {
        println!("hello from a thread");
        Ok(())
    });
    handle.join().unwrap().unwrap();
}
