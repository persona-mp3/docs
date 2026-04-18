fn main() {
    println!("ownerships.rs");
    let s = String::from("original owner of string value");


    // s no longer owns the value but take_ownership
    take_ownership(s);


    // you can longer use s anymore, its gone, Uncomment it and see what happens
    // println!("{s}")
    // 
    //  3 |     let s = String::from("original owner of string value");
    //    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // ...
    //  7 |     take_ownership(s);
    //    |                    - value moved here
    // ...
    // 11 |     println!("{s}")
    //    |                ^ value borrowed here after move
    //    |
    // note: consider changing this parameter type in function `take_ownership` to borrow instead if owning the value isn't necessary
    //   --> src/ownership.rs:16:24


    // Also, simple types implement the 'Copy' trait, and you can also do this 
    // for custom types. The Rust compiler just copies the value into the print_version() 
    // so it doesn't go outofscope
    let version = 20;

    print_version(version);

    println!("version still accessible -> {version}")

}

fn take_ownership(val: String){
    println!("take_ownership is now owner of {val}");
}

fn print_version(version: usize) {
    println!("version -> {version}")
}
