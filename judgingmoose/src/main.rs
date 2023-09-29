use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (l, r) = (parts[0], parts[1]);

    if l == 0 && r == 0 {
        println!("Not a moose");
    } else if l == r {
        println!("Even {}", 2 * l);
    } else {
        let max_tines = if l > r { l } else { r };
        println!("Odd {}", 2 * max_tines);
    }
}
