fn main() {
    let s1 = "petrolleum_jelly";
    let s2 = "what about now Beson?";

    let res = largest(s1, s2);
    println!("Largest string -> {res}",);

    {
        let m1 = "weird_fishes_by: radio_head";
        println!("largest btwn s1 and m1 -> {}", largest(s1, m1));
    }

    let res;
    {
        let m2 = String::from("testing_lifetimes");
        res = largest(s1, &m2.as_str());
    }

    print!("longest -> {res}")
}

fn largest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
