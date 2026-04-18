#![allow(unused)]
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn main() {
    // Multiple producer, single consumer
    let (out_ch, in_ch) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread"),
            String::from("[1]"),
        ];
        for val in vals {
            match out_ch.send(val) {
                Ok(_) => (),
                Err(err) => println!("[1] could not send: {err}"),
            };
        }
    });

    // // The sender sends a close_signal, although not specified by the book my attempt there
    // // was to cause a panic
    // loop {
    //     let recvd = in_ch.recv().expect("Could not read from [1]");
    //     println!("[0] recvd [1]:: {:?}", recvd);
    // }

    for recv in in_ch {
        println!("[0] [1]:: {:?}", recv);
    }
}

#[derive(Debug)]
pub struct Message {
    src: String,
    content: String,
}
pub fn multiple_threads() -> Result<Receiver<Message>, Box<dyn std::error::Error>> {
    let (send, recv) = mpsc::channel();
    let send_clone = send.clone();
    thread::spawn(move || {
        let msgs = vec![
            Message {
                src: String::from("1"),
                content: String::from("My name is Watashiawa no Koshikage"),
            },
            Message {
                src: String::from("1"),
                content: String::from("Bro, wtf is this"),
            },
            Message {
                src: String::from("1"),
                content: String::from(
                    "Can you fist my bump? 
                    - Rocky from `Project Hail Mary`",
                ),
            },
        ];

        for msg in msgs {
            thread::sleep(std::time::Duration::from_millis(4));
            send.send(msg).unwrap()
        }
    });

    let clone_3 = send_clone.clone();
    thread::spawn(move || {
        let msgs = vec![
            Message {
                src: String::from("2"),
                content: String::from("What is bro yapping about?"),
            },
            Message {
                src: String::from("2"),
                content: String::from("Are you sure?"),
            },
            Message {
                src: String::from("2"),
                content: String::from(
                    "What you did was not nice, it was not nice to do that
                    - Einstein, probably",
                ),
            },
        ];

        for msg in msgs {
            thread::sleep(std::time::Duration::from_millis(3));
            send_clone.send(msg).unwrap()
                
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            Message {
                src: String::from("3"),
                content: String::from("Uhhhh..."),
            },
            Message {
                src: String::from("3"),
                content: String::from("Cecil! I need you Cecil!"),
            },
            Message {
                src: String::from("3"),
                content: String::from(
                    "Jolly good show, I say good show lad",
                ),
            },
        ];

        for msg in msgs {
            thread::sleep(std::time::Duration::from_millis(4));
            clone_3.send(msg).unwrap()
        }
    });

    Ok(recv)
}
