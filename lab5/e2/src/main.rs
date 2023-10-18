
use std::io::{BufReader, Read, stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Sender,Receiver,channel};

enum Message{
    Output([u8;200]),
    Input(String),
    //CtrlA(u8),
    Done()
}

fn prompt()->String{
    //PROMPT
    print!("> ");
    let mut cmd_str = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut cmd_str).expect("Did not enter a correct string");
    if let Some('\n') = cmd_str.chars().next_back() {
        cmd_str.pop();
    }
    if let Some('\r') = cmd_str.chars().next_back() {
        cmd_str.pop();
    }
    cmd_str
}

fn main() {

    //let (input_rx,input_tx)=channel();

    loop {
        //EVENT CHANNEL
        let (event_tx,event_rx)=channel();
        let user_tx = event_tx.clone();


        let user_input = std::thread::spawn(move || {
            //let mut s = String::new();

            //stdin().read_line(&mut s).expect("Did not enter a correct string");

            user_tx.send(Message::Input(prompt())).expect("Failed to write to command input!");
        });


        let mut child_option:Option<Child>=Option::None;
        let mut output_recv=Option::None;

        while let Ok(msg) = event_rx.recv() {
            match msg {
                Message::Input(input) => {
                    if input.as_bytes()[0]==1 as u8{
                        if let Option::Some(mut child)=child_option{
                            child.kill().expect("Unable to kill child cmd");
                        }
                        child_option=None;
                        break;
                    }
                    if let Option::None=child_option {
                        let mut child = if cfg!(target_os = "windows") {
                            Command::new("cmd")
                                .args(["/C", &input])
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute process")
                        } else {
                            Command::new("sh")
                                .arg("-c")
                                .arg(input)
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute process")
                        };

                        let output_recv_tx = event_tx.clone();
                        //drop(event_tx);

                        let mut child_stdout = child.stdout.take().expect("Failed to open command stdout!");

                        output_recv = Some(std::thread::spawn(move || {

                            let mut exit_status=child.try_wait();
                            let mut child_buf = BufReader::new(child_stdout);

                            while let Ok(None)=exit_status{
                                let mut s=[0;200];
                                match child_buf.read(&mut s) {
                                    Ok(n) => {
                                        if n>0 {
                                            output_recv_tx.send(Message::Output(s)).expect("Unable to write on event channel");
                                        }
                                    }
                                    Err(err) => {
                                        println!("{}", err);
                                    }
                                };
                                exit_status=child.try_wait();
                            }

                            match exit_status{
                                Result::Err(err)=> {
                                    println!("{}",err);
                                    return;
                                },
                                Result::Ok(Option::None)=>{
                                    println!("output loop failed");
                                }
                                Result::Ok(Option::Some(exit_code))=>{
                                    output_recv_tx.send(Message::Done()).unwrap();
                                }
                            }

                        }));
                        //child_option=Option::Some(child);
                    } else {
                        let mut child_stdin = child_option.as_mut().unwrap().stdin.take().expect("Failed to open command stdin!");
                        child_stdin.write_all(input.as_bytes()).expect("Unable to write to cmd input");
                    }
                },
                Message::Output(output) => {
                    println!("{:?}\n", String::from_utf8_lossy(&output));
                },
                Message::Done()=>{
                    output_recv.unwrap().join().unwrap();
                    break;
                }
            }
        }
    }

}
