use std::io::BufRead as _;

#[derive(Debug)]
enum LemInError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for LemInError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

struct Node {
    name: String,
    x: i64,
    y: i64,
}

struct Map {
    start: Node,
    end: Node,
    nodes: Vec<Node>,
    edges: Vec<(Node, Node)>,
}

impl Map {
    fn parse() -> Result<Self, LemInError> {
        // let mut start = None;
        // let mut end = None;
        // let mut nodes

        // let stdin = std::io::stdin();
        // for line in stdin.lock().lines() {
        //     let line = line?.trim();
        //     match line.as_ref() {
        //         "##start" => "",
        //     }
        // }
        todo!()
    }
}

fn main() {
    let mut map = Map::parse();
}
