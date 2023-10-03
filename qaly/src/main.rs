use std::io;

fn main() {
    let mut left_values = Vec::new();
    let mut right_values = Vec::new();
    let mut addition : f64 = 0.0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let left_value: f64 = input.trim().split_whitespace().nth(0).unwrap().parse().unwrap();
        let right_value: f64 = input.trim().split_whitespace().nth(1).unwrap().parse().unwrap();

        left_values.push(left_value);
        right_values.push(right_value);
    }

    for i in 0..n {
        let result = left_values[i] * right_values[i];
        addition += result;
    }

    println!("{:.3}", addition)

}
