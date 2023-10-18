use std::env::args;
use std::fs::File;
use std::mem::size_of;
use std::io::Read;

#[derive(Debug)]
struct Value{
    data_type:i32,
    val:f32,
    timestamp:i32
}
#[derive(Debug)]
struct MValue {
    data_type: i32,
    val: [f32; 10],
    timestamp: i32
}
#[derive(Debug)]
struct Message{
    data_type:i32,
    message:String
}
/*union Payload{
    val:Value,
    m_val:MValue,
    msg:Message
}*/
#[derive(Debug)]
struct CData{
    data_type:i32,
    //payload:Payload
    val:Value,
    mval:MValue,
    msg:Message
}

impl CData{
    fn from_file(mut fp: &File) ->CData{

        let mut c_data:CData = CData {
            data_type: 0,
            val: Value {
                data_type: 0,
                val: 0.0,
                timestamp: 0
            },
            mval: MValue {
                data_type: 0,
                val: [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.],
                timestamp: 0
            },
            msg: Message { data_type: -1, message: String::from("Ciao papà")}
        };

        let mut buf = [0; 4];
        fp.read_exact(&mut buf);
        c_data.data_type=i32::from_le_bytes(buf);

        //println!("struct type: {}", c_data.data_type);
        if c_data.data_type==0{  //ValueStruct

            fp.read_exact(&mut buf);
            c_data.val.data_type=i32::from_le_bytes(buf);

            //let mut buf = [0;4];
            fp.read_exact(&mut buf);
            c_data.val.val=f32::from_le_bytes(buf);

            //let mut buf = [0;4];
            fp.read_exact(&mut buf);
            c_data.val.timestamp=i32::from_le_bytes(buf);

            let mut buf = [0;36];
            fp.read_exact(&mut buf);

        }
        if c_data.data_type==1{  //MValueStruct
            //let mut buf = [0; 4];
            fp.read_exact(&mut buf);
            c_data.mval.data_type=i32::from_le_bytes(buf);

            for i in 0..10 {
                //let mut buf = [0; 4];
                fp.read_exact(&mut buf);
                c_data.mval.val[i] = f32::from_le_bytes(buf);
            }

            //let mut buf = [0;4];
            fp.read_exact(&mut buf);
            c_data.mval.timestamp=i32::from_le_bytes(buf);

        }
        if c_data.data_type==2{  //MessageStruct
            //let mut buf = [0; 4];
            fp.read_exact(&mut buf);
            c_data.msg.data_type=i32::from_le_bytes(buf);

            let mut str_buf = [0;21]; //Vec::<u8>::with_capacity(21);   //[0;4*12];
            fp.read_exact(&mut str_buf);
            //let str_temp=std::str::from_utf8(&str_buf);
            /*let str_temp = match std::str::from_utf8(&str_buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };*/
            c_data.msg.message=String::from_utf8(Vec::from(str_buf)).unwrap();

            let mut buf = [0;23];
            fp.read_exact(&mut buf);

        }
        c_data
    }
}

fn main() {

    let mut args:Vec<String>=args().skip(1).collect();

    println!("size of an usize {}",size_of::<usize>());
    println!("size of an i32 {}",size_of::<i32>());
    println!("size of an f32 {}",size_of::<f32>());

    if args.len()!=1{
        println!("Missing cmd args!");
        std::process::exit(1);
    }

    let filename=args.remove(0);

    println!("filename: {:?}",filename);

    let fp=File::open(filename).unwrap();

    let mut c_data_vec=Vec::<CData>::new();

    for i in 0..100 {
        c_data_vec.push(CData::from_file(&fp));
    }

    println!("{}",c_data_vec.len());
    for (i,curr_c_data) in c_data_vec.iter().enumerate() {
        println!("{}. {:?}",i,curr_c_data);
    }


    println!("Hello, world!");
}
