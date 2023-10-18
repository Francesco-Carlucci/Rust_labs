use std::thread;
use std::time::Duration;

use e1::{FileBuffer, SensorData};

fn main() {
    let mut data:Vec<SensorData>=Vec::new();

    let mut file_buf=FileBuffer::new("circ_buf.txt",10);
    loop {
        thread::sleep(Duration::from_millis(10_000));  //10s

        let ret=file_buf.import(&mut data);

        println!("Ho letto {} misurazioni",ret);
        println!();

        for d in &data{
            println!("{:?}",d);
        }

        data.clear();

    }
}
