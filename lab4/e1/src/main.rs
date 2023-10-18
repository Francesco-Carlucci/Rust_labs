use std::sync::{Arc};  //Barrier
use std::thread;
use std::time::Duration;

use e1::{ChannelBarrier, CyclicBarrier};

use std::sync::mpsc::{Sender,channel};


fn main() {
    let abarrier= Arc::new(CyclicBarrier::new(3));

    let mut vt=Vec::new();

    for i in 0..3 {
        let cbarrier = abarrier.clone();

        vt.push(thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                //thread::sleep(Duration::from_nanos(1));
                println!("After barrier {}  {}", i, j);
            }
        }));
    }
    for t in vt{
        t.join().unwrap();
    }

    let num_threads=3;
    /*
    let mut sender_vec: Vec<Sender<String>>= Vec::new();
    let mut receiver_vec=Vec::new();
    for _ in 0..3{
        let (tx,rx)=channel();
        sender_vec.push(tx);
        receiver_vec.push(rx);
    }*/
    let channel_barrier=ChannelBarrier::new(num_threads);

    let mut thread_handles =Vec::new();

    for i in 0..3{

        //let thread_senders=sender_vec.clone();
        //let thread_receiver=receiver_vec.remove(0);
        let clonebarrier=channel_barrier.clone();

        thread_handles.push(thread::spawn(move||{

            for _ in 0..8 {

                clonebarrier.wait(i);
                println!("thread n. {i}");
                thread::sleep(Duration::from_millis(1));
                /*
                for s in &thread_senders {
                    s.send(format!("{i}")).unwrap();
                }
                thread::sleep(Duration::from_millis(1));
                for _ in 0..3 {
                    match thread_receiver.recv() {
                        Ok(msg) => {println!("thread n. {i} received {msg}")}
                        Err(err) => {
                            println!("{}", err);
                            panic!("{}", err);
                        }
                    }
                }*/
            }

        }));
    }

    for tid in thread_handles{
        tid.join().unwrap();
    }
}
