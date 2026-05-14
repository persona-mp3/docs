use std::sync::Mutex;

#[derive(Debug)]
struct ChopStick {
    name: String,
}

fn main() {
    let mutex = Mutex::new(ChopStick {
        name: String::from("username is charlie charlie"),
    });

    {
        let mut value = mutex.lock().expect("Could not lock mutex");
        println!("before:: {:?}", value);
        *value = ChopStick {
            name: String::from("wtf is wrong w the neovim config??"),
        };
    }

    let returned_value = mutex.lock().unwrap();
    println!("after:: {:#?}", mutex);
}
