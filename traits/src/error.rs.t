struct Explanation<'a> {
    topic: &'a String,
}

// change name of this file to main.rs or simply copy to main.rs
fn main() {
    let topic = "understanding the static trait".to_string();
    let story = Explanation { topic: &topic };

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("and where were we? oh: {:?}", story.topic);
    });
}
