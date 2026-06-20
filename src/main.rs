use std::io::BufRead as _;

struct Node {}

struct Map {}

impl Map {
    fn parse() {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line.unwrap());
        }
    }
}

fn main() {
    let mut map = Map::parse();
}
