use std::{thread, time::Duration};
use std::fs::File;
use std::io::Write;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[repr(C)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}

fn export(data:[SensorData;10], mut fp: &File){
    fp.write_all(bincode::serialize(data).unwrap());

}

fn main() {
    let mut rng = rand::thread_rng();
    let mut cnt =0;
    let mut data:[SensorData;10];
    loop {
        thread::sleep(Duration::from_millis(1000));

        for mut d in data{
            d.seq=cnt;
            d.values=(0..9).map(|_x| rng.gen::<f32>()).collect(); //gen tra 0 e 1, c'é anche gen_range
            d.timestamp=cnt;
        }
        cnt+=1;




    }
}
