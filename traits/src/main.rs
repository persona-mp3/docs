#![allow(unused)]
use std::error::Error;
fn main() {
    let t = SimTransport {
        connect: sim_connect,
    };
    let f = t.connect;

    broadcast(vec!["".into()], Msg {}, t.connect);
}

struct Msg {}
struct Res {}

type Transport = fn(addr: &str, msg: Msg) -> Result<Res, Box<dyn Error>>;

fn broadcast(peers: Vec<String>, req: Msg, transport: Transport) {}

struct SimTransport {
    connect: Transport,
}

fn sim_connect(addr: &str, msg: Msg) -> Result<Res, Box<dyn Error>> {
    Ok(Res {})
}

trait Network: Send + Sync + 'static + Copy {
    fn connect(&self, addr: String, msg: Msg) -> Result<Res, Box<dyn Error>>;
}

#[derive(Clone, Copy)]
struct TcpNet {}

impl Network for TcpNet {
    fn connect(&self, addr: String, msg: Msg) -> Result<Res, Box<dyn Error>> {
        Ok(Res {})
    }
}
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

struct SimNetwork {
    request: Mutex<HashMap<String, Vec<String>>>,
}

impl Network for SimNetwork {
    fn connect(&self, addr: String, msg: Msg) -> Result<Res, Box<dyn Error>> {
        Ok(Res {})
    }
}

fn mo<N: Network>(n: N) {
    for addr in vec!["".to_string(), "".to_string()] {
        let addr = addr.clone();
        thread::spawn(move || {
            n.connect(addr.clone(), Msg {});
        });
    }
}
