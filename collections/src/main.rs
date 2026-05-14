fn main() {
    let mut names = vec![
        String::from("Dario PrimeTimeAgen"),
        String::from("Kacey Meuratory"),
        String::from("Jonah Hoaesnesbury"),
    ];

    let _item = &names[0];

    names.push(String::from("KawaBanga"));

    println!("Question: {_item}");
}
