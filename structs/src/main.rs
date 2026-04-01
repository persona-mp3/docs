#![allow(dead_code)]
use std::io;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    task: Task,
}

#[derive(Debug)]
/// You can implement methods/functions on an Enum
enum Task {
    Async,
    Ticker,
}

impl Task {
    fn write(&self) {
        println!("testing out enums -> {self:?}",);
    }

    fn run_async(&self) {
        println!("running async function");
    }
}

impl User {
    fn send_notification(&self) {
        println!("sending out notification to email -> {}", self.username);
    }
}

fn main() {
    let tgt_name = collect_target();

    let uname = String::from("persona-mp3");
    let email = String::from("github.com@persona-mp3");

    let mut u = new_user(uname, email);
    change_to(&mut u, tgt_name);
    println!("userdata -> {u:?}");

    u.send_notification();
    u.task.write();

}

fn collect_target() -> String {
    println!("collecting user input");
    println!("Please provide username:");

    let mut tgt_name = String::new();
    match io::stdin().read_line(&mut tgt_name) {
        Ok(_) => {}
        Err(e) => eprintln!("Error occured reading from stdin: {}", e),
    }

    tgt_name
}

fn change_to(u: &mut User, name: String) {
    print!("found username -> {}  || change_to: {}", u.username, name);
    u.username = name;
}

fn new_user(uname: String, email: String) -> User {
    return User {
        active: true,
        email: email,
        username: uname,
        task: Task::Async,
    };
}
