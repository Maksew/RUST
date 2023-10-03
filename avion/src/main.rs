use std::io;

fn main() {
    let mut lines = Vec::new();

    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        lines.push(input.trim().to_string());
    }

    for (index, line) in lines.iter().enumerate() {
        if line.contains("FBI") {
            println!("{}", index + 1)
        }
        if line.contains("CIA") {
            println!("HE GOT AWAY!");
            break;
        }
    }

}
