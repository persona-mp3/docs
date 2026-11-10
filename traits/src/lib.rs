#![allow(unused)]
use std::{error::Error, thread};
pub struct Msg {
    from: String,
    body: String,
}

pub struct Response {
    body: String,
}

mod rack;
mod arc;

/// The [`Clone`] trait is also rather buns in terms of how vague it is, compared
/// to the heavy lifting and how it's always slapped with the [`Copy`] trait.
/// Clone means that the type T can always return a copy of the value of type T
/// { wonder if this is how math people talk or throw stones at each other }
/// But in accordance to the docs: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
/// ...it depends
/// T -> T expensive copy
/// &T -> &T so you don't really own the value, which...make sense
/// Arc,Rc -> increments the ref counts, but you get the &mut T
pub trait Transport: Send + Clone + 'static + Copy {
    fn dial(&self, addr: &str, msg: Msg) -> Result<Response, Box<dyn Error>>;
}

#[derive(Clone, Copy)]
struct JsonTcpTransport {}

impl Transport for JsonTcpTransport {
    fn dial(&self, addr: &str, msg: Msg) -> Result<Response, Box<dyn Error>> {
        Ok(Response {
            body: "".to_string(),
        })
    }
}

/// T: 'static trait bound means that the type owns all the data is contains.
/// ie it doesn't contain references to other values that might be dropped.
/// And whoever uses T has all those values. for example
/// struct Explanation { topic: String } satisfies the 'static trait
///
/// but struct Explanation<'a> { topic: &'a String } where the inner value could be
/// dropped at anytime. For example
///    struct Explanation<'a> {
///        topic: &'a String,
///    }
///
///    fn main() {
///       let topic = "understanding the static trait".to_string();
///       let story = Explanation { topic: &topic };
///
///       std::thread::spawn(move || {
///           std::thread::sleep(std::time::Duration::from_secs(3));
///           println!("and where were we? oh: {:?}", story.topic);
///       });
///    }
///
/// While main's stack frame has been cleaned and popped, along with it's
/// stack data ie `topic`, thread might have then just woken just to see a cleaned
/// out spot. It's like you playing hide and seek with your friends, but they left
/// you when you counted to 100. You see no-one, hear-one
///
/// Okay so the compiler can prove that your type, T is suitable to impl 'static and
/// that is just by checking if T owns it's data
fn connector<T: Transport>(peers: &Vec<String>, port: T) {
    for addr in peers.iter() {
        let addr = addr.clone();
        let msg = Msg {
            from: "".to_string(),
            body: "".to_string(),
        };
        thread::spawn(move || {
            port.dial(addr.as_str(), msg);
        });
    }
}
