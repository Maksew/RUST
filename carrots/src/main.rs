use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let parts: Vec<&str> = input.split_whitespace().collect();
    let p: i32 = parts[1].parse().expect("Erreur de conversion");

    println!("{}", p);
}
