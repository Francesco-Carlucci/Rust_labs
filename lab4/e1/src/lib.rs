use std::sync::{Condvar, Mutex};


pub struct CyclicBarrier{
    leaved:(Mutex<usize>,Condvar),
    flag:(Mutex<bool>,Condvar),
    arrived:Mutex<usize>,
    num_threads:usize,
}

impl CyclicBarrier{
    pub fn new(num_threads:usize)->CyclicBarrier{
        CyclicBarrier{leaved:(Mutex::new(num_threads),Condvar::new()),
            flag:(Mutex::new(false),Condvar::new()),
            arrived:Mutex::new(0),
            num_threads:num_threads
            }
    }

    pub fn wait(&self){
        let mut arrived=self.arrived.lock().unwrap();
        if *arrived==0 {   //primo thread nella barriera
            let (leaved_lock, leaved_cvar) = &self.leaved;
            let  mut leaved=leaved_lock.lock().unwrap();

            if *leaved==self.num_threads{     //tutti se ne sono andati
                let mut flag=self.flag.0.lock().unwrap();
                *flag=false;
                //self.flag.1.notify_all();
            } else{
                //drop(arrived);
                while *leaved != self.num_threads {
                    leaved=leaved_cvar.wait(leaved).unwrap(); //wait for all to leave
                }
                let mut flag=self.flag.0.lock().unwrap();
                *flag=false;
                //self.flag.1.notify_all();
            }
        }
        //let mut arrived=self.arrived.lock().unwrap();
        *arrived+=1;
        drop(arrived);

        let mut arrived=self.arrived.lock().unwrap();
        if *arrived==self.num_threads{
            *arrived=0;
            drop(arrived);
            let  mut leaved=self.leaved.0.lock().unwrap();
            *leaved=1;

            let mut flag=self.flag.0.lock().unwrap();
            *flag=true;
            self.flag.1.notify_all();
        }else{
            drop(arrived);
            let mut flag=self.flag.0.lock().unwrap();
            while *flag==false{
                flag=self.flag.1.wait(flag).unwrap();
            }
            let  mut leaved=self.leaved.0.lock().unwrap();
            *leaved+=1;
            if *leaved==self.num_threads{self.leaved.1.notify_all();}
        }

    }
}

use std::sync::mpsc::{Sender,Receiver,channel};
use std::sync::Arc;

#[derive(Clone)]
pub struct ChannelBarrier{
    sender_vec:Vec<Sender<String>>,
    receiver_vec:Arc<Vec<Mutex<Receiver<String>>>>,
    num_threads:usize,
}

impl ChannelBarrier{

    pub fn new(num_threads:usize)->ChannelBarrier{
        let mut sender_vec: Vec<Sender<String>>= Vec::new();
        let mut receiver_vec=Vec::new();
        for _ in 0..num_threads{
            let (tx,rx)=channel();
            sender_vec.push(tx);
            receiver_vec.push(Mutex::new(rx));
        }
        ChannelBarrier{sender_vec,receiver_vec:Arc::new( receiver_vec),num_threads}
    }

    pub fn wait(&self,i:usize){
        //println!("thread n. {i} entered the barrier");

        for s in &self.sender_vec {
            s.send(format!("{i}")).unwrap();
        }

        let rec_lock=self.receiver_vec[i].lock().unwrap();
        for _ in 0..self.num_threads{
            match rec_lock.recv(){
                Ok(msg) => {/*println!("thread n. {i} received {msg}")*/}
                Err(err) => {
                    println!("{}", err);
                    panic!("{}", err);
                }
            }
        }


    }
}