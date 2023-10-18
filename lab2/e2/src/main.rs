
use e2::{FileSystem,File,FileType};



fn main() {
    let file_sys=FileSystem::from_dir(
        "C:/Users/utente/Desktop/Programmazione di sistema/Rust/lab23/lab2/e2");

    println!("{:?}",file_sys);

    if FileSystem::mk_dir("C:/Users/utente/Desktop/Programmazione di sistema/Rust/prova1"){
        println!("directory created")
    }else{println!("parent directory doesn't exist")};

    if FileSystem::rm_dir("C:/Users/utente/Desktop/Programmazione di sistema/Rust/prova1"){
        println!("directory removed")
    }else{println!("directory removal failed!")};

    FileSystem::mk_dir("C:/Users/utente/Desktop/Programmazione di sistema/Rust/prova1");

    let new_file=File::new("prova2.txt".to_string(),
                           "Ciao mamma!".as_bytes().to_vec(),
    64,FileType::Binary);

    FileSystem::new_file(
        "C:/Users/utente/Desktop/\
        Programmazione di sistema/Rust/prova1",
        new_file
    );




}
