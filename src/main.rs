struct Node {
    name: String,
    x: i64,
    y: i64,
}

struct Map {
    ants: u64,
    start: Node,
    end: Node,
    nodes: Vec<Node>,
    edges: Vec<(Node, Node)>,
}

impl Map {
    const SUBJECT1: Self = todo!();
}

fn main() {
    let mut map = Map::SUBJECT1;
}
