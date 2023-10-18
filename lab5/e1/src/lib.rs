
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool<F>{
    jobs:Arc<(Mutex<VecDeque<F>>,Condvar)>,
    num_workers:usize,
    workers:Vec<JoinHandle<()>>
}

impl<F: FnOnce()->() + Send +'static> ThreadPool<F>{
    pub fn new(num_workers:usize)->ThreadPool<F>{
        let jobs=Arc::new((Mutex::new(VecDeque::new()),Condvar::new()));
        let mut workers=Vec::new();
        for i in 0..num_workers{
            let local_jobs=jobs.clone();
            workers.push(thread::spawn(move ||{ Self::threadfunc(local_jobs,i) }));
        }
        ThreadPool{jobs:jobs,
            num_workers:num_workers,
            workers:workers
        }
    }
    pub fn execute(&self, job: F) {
        //let (queue_lock,queue_var)=self.jobs;
        let mut queue=self.jobs.0.lock().unwrap();
        queue.push_back(job);
        self.jobs.1.notify_all();
    }

    fn threadfunc(jobs: Arc<(Mutex<VecDeque<F>>,Condvar)>, i:usize){
        let queue_lock=&jobs.0;
        let queue_var=&jobs.1;
        loop {
            let mut queue = queue_lock.lock().unwrap();

            while queue.is_empty() {
                queue = queue_var.wait(queue).unwrap();
            }
            let my_job = queue.pop_front().unwrap();
            drop(queue);

            my_job();
        }
    }
}