use std::{thread, time::Duration};

use rand::prelude::*;

use e1::{FileBuffer,SensorData};


fn main() {
    let mut rng = rand::thread_rng();
    let mut cnt =0;
    //let mut data:[SensorData;10];

    let mut file_buf=FileBuffer::new("circ_buf.txt",20);
    loop {
        thread::sleep(Duration::from_millis(1000));

        //for _i in 0..=9{
        let d=SensorData{seq:cnt,
            values:(0..=9).map(|_x| rng.gen::<f32>()).collect::<Vec<f32>>().try_into().unwrap(),
            timestamp:cnt};

        let success=file_buf.export(d);
        if !success{println!("Write failed, buffer full");}
        //}
        cnt+=1;

    }
}
