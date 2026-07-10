use std::sync::LazyLock;

type Edge = (Node, Node);

struct Node {
    name: String,
    // TODO: position for visualizer
    // x: i64,
    // y: i64,
}

impl Node {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

struct Map {
    ants: u64,
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    start: usize,
    end: usize,
}

static MAP_SUBJECT_1: LazyLock<Map> = LazyLock::new(|| Map {
    ants: 3,
    nodes: vec![Node::new("0"), Node::new("1"), Node::new("2"), Node::new("3")],
    edges: vec![vec![2], vec![3], vec![3], vec![1, 2]],
    start: 0,
    end: 1,
});

fn main() {}
