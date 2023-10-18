use std::{thread, time::Duration};
use rand::prelude::*;  //for random data generation
use std::sync::Arc;

use e2::{RingBuf,SensorData};


fn producer(ring_buf: Arc<RingBuf<SensorData>>){
    let mut rng = rand::thread_rng();
    let mut cnt =0;

    loop {
        thread::sleep(Duration::from_millis(1000));

        let d=SensorData{seq:cnt,
            values:(0..=9).map(|_x| rng.gen::<f32>()).collect::<Vec<f32>>().try_into().unwrap(),
            timestamp:cnt};

        match ring_buf.write(d){
            Ok(())=>{println!("Successfully wrote on buffer!")}
            Err(())=>{println!("Buffer full!")}
        }

        cnt+=1;

    }
}

fn consumer(ring_buf: Arc<RingBuf<SensorData>>){

    loop {
        thread::sleep(Duration::from_millis(10_000));  //10s

        match ring_buf.read(){
            Some(data)=> {println!("Read {:?} measurements:\n {:?}",data.len(),data)}
            None =>{println!{"Buffer is empty!"}}
        }
    }
}


fn main() {

    let ring_buf=Arc::new(RingBuf::new(11));

    thread::scope(|s|{
        let prod_buf=ring_buf.clone();
        let cons_buf=ring_buf.clone();
        s.spawn(move||{producer(prod_buf)});
        s.spawn(move||{consumer(cons_buf)});
    });

    //let producer=thread::spawn(||{producer(ring_buf.clone())});

    //producer.join().unwrap();

}
