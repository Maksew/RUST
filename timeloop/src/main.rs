use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    let n: i32 = input.trim().parse().expect("Entrée invalide");

    for i in 1..=n {
        println!("{} Abracadabra", i);
    }
}
