// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main_2() {
    println!("stack vs heap debacle");
    {
        let s = String::from("hoe lee sheet");
        let s2 = s;

        // This would cause an error because s is considered
        // invlaid and the compiler assumes it 'dropped' because the 
        // mantle of ownership of String::from("hoo lee sheet") 
        // has been passed onto s2. This is it to avoid Double Free 
        // memory? or dangling pointers? Because when you copy values like 
        // a String, it copies the pointer to the value of the string in the 
        // heap. So you could imagine trying to free(s) first, using s2 anywhere 
        // would cause UB, and a dangling pointer. It almost makes sense
        // how `defer` would come into play here? But that doesn't remove the whole 
        // error
        // ================= UNCOMMENT THIS LINE TO GET THE ERROR BLASTED ==========================
        // println!("{s} should will throw some errors because {s} is dirty and no longer owns the value from the heap");
        //
        // Heres the error:
        //
        // error[E0382]: borrow of moved value: `s`
        //   --> src/main.rs:18:20
        //    |
        //  4 |         let s = String::from("hoe lee sheet");
        //    |             - move occurs because `s` has type `String`, which does not implement the `Copy` trait
        //  5 |         let s2 = s;
        //    |                  - value moved here
        // ...
        // 17 |         println!("{s} should will throw some errors because {s} is dirty and no longer owns the value from the heap");
        //    |                    ^ value borrowed here after move
        //    |
        //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly buil
        // ds, run with -Z macro-backtrace for more info)
        // help: consider cloning the value if the performance cost is acceptable
        println!("{s2} will work because it now owns the string");
    }

    {
        // Now you might ask well, how do I just copy the item. Well, Strings by their 
        // nature are dynamic. You can't really see this in alot of languages, except you've 
        // worked in C or Zig before. So copying is actually a performance bottleneck, as it 
        // really duplicates that value on the heap. And that means more GC and less performance
        // But some languages like Go and others aformentioned allow you to pass values by 
        // reference.
        // Something like this in Go
        // bigString := "very_fat_string_very_long_imagine"
        // func work_with_string(s *string){}
        //
        // func work_with_string(a *string) {
        // 	println("working_with_string", *a)
        // }
        // func main() {
        // 	s := "very_fat_string_imaging_it_'s_big"
        // 	work_with_string(&s)
        // }
        
        // By cloning
        let original = String::from("original string");
        let copy = original.clone();
        println!("orignial string -> {original}");
        println!("copied string -> {copy}");


        // NOTE compared to primitive types, they simply copy 
        // it beacuse they're gaurantted at compile time. For example 
        // ints, booleans are really small, and cheap to copy


        // read ownerships.rs for more
    }
}

fn main() {
    let mut s = String::from("doctor house is kinda interseting");
    do_something(&mut s); // so the ownership doesn't really go out the scope
    println!("mutated string -> {s}")
}

fn do_something(s : &mut String) {
    s.push_str(" s1_e6");
    println!("has mutable reference to s -> {s}");
}


