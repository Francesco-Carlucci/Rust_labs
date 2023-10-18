use std::sync::{Arc,Mutex};

#[derive(Clone,Debug)]
pub struct SensorData {
    pub seq: u32, // sequenza letture
    pub values: [f32; 10],
    pub timestamp: u32
}

pub struct CircBuf<T>{
    n:usize,
    head:usize,
    tail:usize,
    vec:Vec<T>
}

pub struct RingBuf<T>{
    buf:Arc<Mutex<CircBuf<T>>>
}

impl<T:Clone> RingBuf<T> {
    pub fn new(n:usize)->RingBuf<T>{
        RingBuf{buf:Arc::new(Mutex::new(CircBuf{n:n,head:0,tail:0,
            vec:Vec::with_capacity(n as usize)}))}
    }

    pub fn read(& self)->Option<Vec<T>>{
        let mut v=self.buf.lock().unwrap();

        let mut data=Vec::new();

        while v.tail!=v.head{
            data.push(v.vec[v.tail].clone());
            v.tail=(v.tail+1)%v.n;

        }
        if data.len()==0{
            return Option::None;
        }
        Option::Some(data)
    }

    pub fn write(&self, val:T )->Result<(),()>{
        let mut v=self.buf.lock().unwrap();
        let next_head= (v.head+1)%v.n;
        if next_head!=v.tail{
            let head=v.head;
            v.vec.insert(head,val);
            v.head=next_head;
            return Result::Ok(());
        }
        return Result::Err(());
    }
}