#![allow(unused)]
use std::thread;
use std::time::Duration;

pub fn main() {
    let handle = thread::spawn(|| {
        for i in 0..12 {
            println!("[1] -> {i}");
            thread::sleep(Duration::from_secs(i % 2));
        }
    });

    let static_string = "static string";
    let current_track = vec!["wierd fishes by Radiohead"];
    let handle_2 = thread::spawn(move || {
        // println!("[2] current_track: {:?}", current_track);
        let clone_cp = current_track.clone();
        takes_ownership(&clone_cp);
        // This works even without the move because `static_string` has the
        // lifetime of the whole program. I understand why the borrow checker wants
        // to make sure all  threads access values that will forever be valid
        try_lifetimes(static_string);
    });

    for i in 0..5 {
        println!("[0] :: {i}");
        thread::sleep(Duration::from_secs(i));
    }

    // handle.join is blocking
    // In some sense, this is the equivalent of using
    // `wg.Wait()` and `go func () {//code}` () in Go
    match handle.join() {
        Ok(res) => println!("result from spawned thread: {:?}", res),
        Err(err) => println!("error occured while joinning: {:?}", err),
    };

    match handle_2.join() {
        Ok(res) => println!("result from spawned thread_2: {:?}", res),
        Err(err) => println!("error occured while joinning thread_2: {:?}", err),
    };
}

fn takes_ownership(v: &Vec<&str>) {
    println!("ownership -> {:?}", v);
}

fn try_lifetimes(v: &str) {
    println!("using lifetime -> {:?}", v);
}
