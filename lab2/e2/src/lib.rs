
use std::fs::{read_dir,create_dir,remove_dir,OpenOptions};
//use filetime::{set_file_mtime};

use std::io::{Read, Write};
use std::os::windows::fs::MetadataExt;

use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    Text, Binary
}
#[derive(Debug)]
pub struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}
impl File{
    pub fn new(name:String, content:Vec<u8>, creation_time:u64, type_:FileType) ->File{
        File{name,content,creation_time,type_}
    }
}
/*
impl Debug for File{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name+self.creation_time
    }
}*/
#[derive(Debug)]
struct Dir {
    name: String,
    creation_time: u64,
    children: Vec<Node>,
}
#[derive(Debug)]
enum Node {
    File(File),
    Dir(Dir),
}
#[derive(Debug)]
pub struct FileSystem {
    root: Dir
}


impl FileSystem{
    pub fn new()->FileSystem{
        FileSystem{root:Dir{name:String::new(),creation_time:0,children:Vec::new()}}
    }

    pub fn from_dir(path:&str)->FileSystem{
        let mut path:PathBuf=PathBuf::from(path);   //converte da stringa a PathBuf

        FileSystem{root:FileSystem::explore_dir(path)}     //instantiate our filesystem
    }

    fn explore_dir(path:PathBuf)->Dir{
        let mut metadata = std::fs::metadata(&path).unwrap();

        let mut filename=String::from(path.file_name().unwrap().to_str().unwrap());

        let mut curr_dir=Dir{name:filename,
                            creation_time:metadata.creation_time(),
                            children:Vec::new()};

        let entries=read_dir(path).unwrap();

        for entry in entries{
            let en_path=entry.unwrap().path();
            metadata=std::fs::metadata(&en_path).unwrap();

            if metadata.is_file(){
                filename=String::from(en_path.file_name().unwrap().to_str().unwrap());

                let mut buf:[u8;1000]=[0;1000];

                let mut fp=std::fs::File::open(en_path).expect("Unable to open file!"); //OpenOptions::new().read(true).open(en_path).unwrap();
                fp.read(&mut buf).expect("Unable to read the file!");

                curr_dir.children.push(Node::File(
                    File{name:filename,
                        content:Vec::from("content"),  // mettere buf qui
                        creation_time:metadata.creation_time(),
                        type_:FileType::Binary})
                );


            }else if metadata.is_dir(){
                curr_dir.children.push(Node::Dir(Self::explore_dir(en_path)));
            }
        }
        curr_dir
    }

    pub fn mk_dir(path:&str)->bool{
        let mut path:PathBuf=PathBuf::from(path);

        if path.parent().unwrap().exists(){
            create_dir(path);
            return true;
        }else{
            return false;
        }
    }
    pub fn rm_dir(path:&str)->bool{
        let mut path:PathBuf=PathBuf::from(path);
        if !path.exists(){
            println!("Directory does not exists");
            return false;
        }
        let is_empty=read_dir(&path).unwrap().count()==0;

        if is_empty{
            return match remove_dir(path){
                Ok(t)=> true,
                Err(e)=>{println!("{}",e);return false;}
            };
        }
        println!("Directory {} is not empty",path.display());
        false
    }

    pub fn new_file(path:&str,file:File){
        let mut filepath=PathBuf::from(path);
        filepath.push(file.name);
        let mut fp=OpenOptions::new().create(true).write(true).open(&filepath)
            .expect("File creation failed!");
        //set_file_mtime(filepath,file.creation_time).unwrap();
        fp.write(&file.content);
    }
}