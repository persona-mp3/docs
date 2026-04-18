fn main() {
    let mut s = String::from("orginal_value_");

    // Now while reading the docs, I can't seem to pinpoint
    // exactly while this an error, and I haven't looked at it too on the 
    // docs. But here's what I can guess from experience. When working 
    // with concurrency, this is going to bang your application up. 
    // If you have two possible threads trying to mutate something, that's dagerous 
    // and this is what's going on here, or what the compile is trying to prevent by default 
    // So if you habve two mutable references, compiler is like, `hold on...what is it you're 
    // trying to do here`? But more often that not, you'd actually want to do something 
    // that involves two different things to the same 'source'
    // Well something I can think of is to give one a copy and another one just ref it and the 
    // kinda like diff | aggregate the results, like how you would use GoChannels

    //  ========= UNCOMMENT IT TO LET IT SCREAM =========
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}");

    //
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //   --> src/borrow.rs:27:14
    //    |
    // 26 |     let r1 = &mut s;
    //    |              ------ first mutable borrow occurs here
    // 27 |     let r2 = &mut s;
    //    |              ^^^^^^ second mutable borrow occurs here
    // ...
    // 30 |     println!("{r1}");
    //    |                -- first borrow later used here

    // warning: unused variable: `r2`
    //   --> src/borrow.rs:27:9
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html 
    // Now I read the docs, it's kinda the same point the were going after, they said you could
    // simply just give them another scope

    // But this will always run first though

    {
        let r1 = &mut s;
        r1.push_str("first_mutation_");
    }

    let r2 = &mut s;
    r2.push_str("second_mutation_");
    println!("{r2}");





    // This will also cause an error, another way I can imagine why this 
    // was included in the compiler design is Imagine you have two threads, that 
    // are looking at the same value, if one changes it, the other threads is 
    // most likely operating on stale data, So insome sense, is why the designed 
    // the compiler like this? You cant have two values holding a mutable ref
    // and an immutable ref. It's like 'I trusted you, how can you do this to me`
    // It's almost as if Rust wasn't meant for normal applications.
    let mut username = String::from("persona-mp3");
    let b = &mut username;
    let a = &username;
    
    b.push_str("mutated_usersname");

}
