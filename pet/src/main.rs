use std::io;

fn main() {
    let mut un_values = Vec::new();
    let mut deux_values = Vec::new();
    let mut trois_values = Vec::new();
    let mut quatre_values = Vec::new();
    let n = 5;

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<f64> = input.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        un_values.push(parts[0]);
        deux_values.push(parts[1]);
        trois_values.push(parts[2]);
        quatre_values.push(parts[3]);
    }

    let mut max_points: f64 = 0.0;
    let mut gagnant: usize = 0;
    for i in 0..n {
        let total_points = un_values[i] + deux_values[i] + trois_values[i] + quatre_values[i];
        if total_points > max_points {
            max_points = total_points;
            gagnant = i + 1;
        }
    }

    println!("{} {:.0}", gagnant, max_points);
}
