use e1::ThreadPool;
use std::thread;
use std::time::Duration;


fn main() {
    let threadpool=ThreadPool::new(10);

    for x in 0..100{
        threadpool.execute(move || {
            println!("long running task {}", x);
            thread::sleep(Duration::from_millis(10000))
        });

        //thread::sleep(Duration::from_millis(900));
    }


    // just to keep the main thread alive
    loop {thread::sleep(Duration::from_millis(1000))};

}
