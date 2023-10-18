use crossbeam::channel::{bounded, Receiver, Sender};
use std::{marker::PhantomData, thread};
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
enum State {
    Entering(usize),
    Exiting(usize),
}

fn is_entering(state: &State) -> bool {
    match state {
        State::Entering(_) => true,
        _ => false,
    }
}

pub struct CyclicBarrier {
    size: usize,
    mutex: Mutex<State>,
    cond: Condvar,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        CyclicBarrier {
            size: n,
            mutex: Mutex::new(State::Entering(0)),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut state = self.mutex.lock().unwrap();
        state = self.cond.wait_while(state, |s| !is_entering(s)).unwrap();

        // we are entering
        if let State::Entering(n) = *state {
            if n == self.size - 1 {
                *state = State::Exiting(self.size - 1);
                self.cond.notify_all();
            } else {
                *state = State::Entering(n + 1);
                state = self.cond.wait_while(state, |s| is_entering(s)).unwrap();
                if let State::Exiting(n) = *state {
                    if n == 1 {
                        // the last one set state to entering
                        *state = State::Entering(0);
                        self.cond.notify_all();
                    } else {
                        *state = State::Exiting(n - 1);
                    }
                } else {
                    panic!("unexpected state");
                };
            }
        } else {
            panic!("unexpected state");
        }
    }
}

type Channel = (Sender<()>, Receiver<()>);

pub struct ChannelBarrier {
    channels: Vec<Channel>,
}

impl ChannelBarrier {
    pub fn new(n: usize) -> Self {
        let mut channels = Vec::new();
        for _ in 0..n {
            channels.push(bounded::<()>(n));
        }

        ChannelBarrier { channels }
    }

    pub fn wait(&self, i: usize) {
        for (s, _) in &self.channels {
            s.send(()).unwrap();
        }

        let (_, r) = &self.channels[i];
        for _ in 0..self.channels.len() {
            r.recv().unwrap();
        }
    }
}

// heare the barrier contains only the endpoints between a single and the supervisor
// the new() function spawns the supervisor thread and retruns a vector of barriers
// we also added a return value and a generic computation function passed to teh supervisor

pub struct SupervisorBarrier<T, F> {
    wait_sender: Sender<T>,       // from each thread to supervisor
    result_receiver: Receiver<T>, // from supervisor to each thread
    phantom: PhantomData<F>,      // since we don't actually need F here
}

impl<T, F> SupervisorBarrier<T, F>
where
    T: Send + 'static + Copy, // we need Copy to send back the value to each thread
    F: Fn(Vec<T>) -> T + Send + 'static,
{
    pub fn new(n: usize, op: F) -> Vec<Self> {
        let (wait_sender, wait_receiver) = bounded::<T>(n);

        let mut result_senders = Vec::new();
        let mut result_receivers = Vec::new();

        for _ in 0..n {
            let (s, r) = bounded::<T>(1);
            result_senders.push(s);
            result_receivers.push(r);
        }

        thread::spawn(move || {
            loop {
                let mut results = Vec::new();

                // wait n times
                for i in 0..n {
                    match wait_receiver.recv() {
                        Ok(x) => {
                            results.push(x);
                        }
                        Err(_) => {
                            return;
                        }
                    }
                }

                let res = op(results);
                for s in result_senders.iter() {
                    s.send(res);
                }
            }
        });

        let mut barriers = Vec::new();
        for i in 0..n {
            barriers.push(SupervisorBarrier {
                wait_sender: wait_sender.clone(),
                result_receiver: result_receivers.pop().unwrap(),
                phantom: PhantomData,
            })
        }
        barriers
    }

    pub fn wait(&self, val: T) -> T {
        self.wait_sender.send(val);
        return self.result_receiver.recv().unwrap();
    }
}
