use std::io;
use std::io::BufRead;


struct Cube {
    number: i32,
    color: Colors
}

enum Colors {
    Red,
    Green,
    Blue
}

impl Cube {


}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => {
                eprintln!("Error reading line {}", e);
                break;
            }
        }
    }


}
