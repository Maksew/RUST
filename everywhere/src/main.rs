use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut cities = HashSet::new();

        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            cities.insert(input.trim().to_string());
        }

        println!("{}", cities.len());
    }
}
