mod channels;
mod threads;

fn main() {
    let out = match channels::multiple_threads() {
        Ok(recvr) => recvr,
        Err(err) => {
            println!("error: {err}");
            return;
        }
    };
    for msg in out {
        println!("{:?}", msg);
    }
}
