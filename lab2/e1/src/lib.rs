//file management
//use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Write,Read, Seek, SeekFrom};
use std::path::Path;
//serialization
use serde::{Serialize, Deserialize};
extern crate bincode;
//synchronization
use fcntl::{lock_file, unlock_file, FcntlLockType};

#[derive(Serialize, Deserialize,Debug)]
#[repr(C)]
pub struct SensorData {
    pub seq: u32, // sequenza letture
    pub values: [f32; 10],
    pub timestamp: u32
}
impl SensorData{
    fn size_of()->u64{
        return std::mem::size_of::<SensorData>() as u64;
    }
}
#[derive(Serialize, Deserialize,Debug)]
#[repr(C)]
struct Header{
    n:u64,
    head:u64,
    tail:u64
}

impl Header{
    fn size_of()->usize{
        return std::mem::size_of::<Header>();
    }
}

const HEADER_OFFSET: u64 =24;
//const SDATA_SIZE: u64=48;

#[derive(Debug)]
pub struct FileBuffer{
    n:u64,
    fp:std::fs::File,
    limes:u64,
}

impl FileBuffer{

    pub fn new(filename:&str,n:u64)->FileBuffer{
        println!("Header size:{},SensorData size:{}",Header::size_of(),SensorData::size_of());
        let mut new=true;
        if Path::new(filename).exists(){
            new=false;
        }
        let file=OpenOptions::new().read(true).write(true)
                                    .create(true).open(filename)
                                    .unwrap();

        /*let file=match File::open(path){
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };*/

        let mut file_buf =FileBuffer{n:n,fp:file,limes:n*SensorData::size_of()};  //,head:HEADER_OFFSET,tail:HEADER_OFFSET

        if new{
            let new_header=Header{head: HEADER_OFFSET,tail:HEADER_OFFSET,n:n};
            file_buf.write_header(new_header);
        }
        else{
            let curr_header=file_buf.read_header();
            file_buf.n=curr_header.n;
            file_buf.limes=curr_header.n*SensorData::size_of();
        }
        file_buf
    }

    fn read_header(&mut self)->Header{
        let  mut buf=[0;24]; //24 bytes di Header struct

        self.fp.seek(SeekFrom::Start(0)).expect("Unable to rewind file");
        self.fp.read_exact(&mut buf).expect("Unable to read file");

        let curr_header:Header=bincode::deserialize(&buf).unwrap();
        //self.n=curr_header.n;
        //self.head=curr_header.head;
        //self.tail=curr_header.tail;
        curr_header
    }

    fn write_header(&mut self,header:Header){
        self.fp.seek(SeekFrom::Start(0)).expect("Unable to rewind");
        self.fp.write(&bincode::serialize(&header).unwrap()).expect("Unable to write");

    }

    pub fn export(&mut self,data:SensorData)->bool{
        let mut success=true;

        //spinlock
        while !match lock_file(&self.fp, None, Some(FcntlLockType::Write)) {
            Ok(true) => {/*println!("Lock acquired!");*/ true},
            Ok(false) => {println!("Could not acquire lock!"); false},
            Err(err) => {println!("Error: {:?}", err); false},
        }{}

            let mut curr_header=self.read_header();

            let next_head=(curr_header.head-HEADER_OFFSET+SensorData::size_of())%self.limes+HEADER_OFFSET;
            //println!("curr_header: {:?}, next head: {}",curr_header,next_head);
            //println!("current data: {:?}",data);

            if next_head!=curr_header.tail {

                self.fp.seek(SeekFrom::Start(curr_header.head)).expect("Unable to jump at head position");
                self.fp.write(&bincode::serialize(&data).unwrap()).expect("Unable to write to file!");   //&bincode translate from Vec<u8> to &[u8]
                curr_header.head = next_head;

                self.write_header(curr_header);
            } else{
                success=false;
            }

        match unlock_file(&self.fp, None) {
            Ok(true) => (),//println!("Lock successfully released"),
            Ok(false) => println!("Failed to release lock"),
            Err(err) => println!("Error: {:?}", err),
        }

        return success;
    }

    pub fn import(&mut self,data:&mut Vec<SensorData>)->usize{

        while !match lock_file(&self.fp, None, Some(FcntlLockType::Read)) {
            Ok(true) => {/*println!("Lock acquired!");*/ true},
            Ok(false) => {println!("Could not acquire lock!"); false},
            Err(err) => {println!("Error: {:?}", err); false},
        }{}

            let mut curr_header=self.read_header();
            println!("current header: {:?}",curr_header);

            let mut buf=[0;48];  //48 é SensorData::size_of() as usize

            let mut cnt=0;

            while curr_header.tail!=curr_header.head {

                self.fp.seek(SeekFrom::Start(curr_header.tail)).expect("Unable to jump at tail position");

                self.fp.read_exact(&mut buf).expect("Unable to read from file");
                data.push(bincode::deserialize(&buf).unwrap());
                curr_header.tail = ( curr_header.tail-HEADER_OFFSET + SensorData::size_of() ) % self.limes+HEADER_OFFSET;
                cnt+=1;
            }

            self.write_header(curr_header);

        match unlock_file(&self.fp, None) {
            Ok(true) => (),//println!("Lock successfully released"),
            Ok(false) => println!("Failed to release lock"),
            Err(err) => println!("Error: {:?}", err),
        }

        cnt  //numero di misurazioni lette

    }

}