#![warn(unused_assignments)]
use std::cmp::Ordering;
use std::io;

use rand::Rng;


const CONSTANTS_SHOULD_HAVE_UPPERCASE_NAMES :u32 = 90 * 90;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//
//
//
//
//
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
//
// fn main() {
//
//     let secret_number = rand::thread_rng().gen_range(31..=79);
//
//     loop {
//         println!("please provide a number");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("somethings off");
//
//         let user_guess: Result<u32, _> = guess.trim().parse();
//
//         match user_guess {
//             Ok(num) => num,
//             Err(e) => continue,
//             // Ok(val) => {
//             //     let res = show_results(val, secret_number);
//             //     if !res {
//             //         continue
//             //     }
//             //     return
//             // }    ,
//             // Err(e) => {
//             //         println!("Error happend -> {e}");
//             }
//
//         // show_results(user_guess, secret_number)
//     }
//
// } 
//
// // fn show_results(val: u32, tgt: u32) -> bool {
//     // match val.cmp(&tgt) {
//     //     Ordering::Less =;> {
//     //        println!("rookie numbers");
//     //         return false
//     //     },
//     //     Ordering::Equal => {
//     //        println!("You wan Hoo Lee Shee");
//     //         return true;
//     //     },
//     //     Ordering::Greater => {
//     //        println!("take a chill pill brother");
//     //         return false;
//     // }
//
// // }
