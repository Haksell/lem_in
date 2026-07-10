use std::{collections::VecDeque, sync::LazyLock};

type Edge = (Node, Node);
type Output = Vec<Vec<Move>>;

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

struct Move {
    ant: usize,
    room: usize,
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

static MAP_SUBJECT_3: LazyLock<Map> = LazyLock::new(|| Map {
    ants: 3,
    nodes: vec![
        Node::new("3"),     // 0
        Node::new("start"), // 1
        Node::new("end"),   // 2
        Node::new("4"),     // 3
        Node::new("1"),     // 4
        Node::new("2"),     // 5
        Node::new("5"),     // 6
        Node::new("6"),     // 7
    ],
    edges: vec![
        vec![1, 3],
        vec![0, 4],
        vec![5, 7],
        vec![0, 5],
        vec![1, 5, 6],
        vec![2, 3, 4],
        vec![4, 7],
        vec![2, 6],
    ],
    start: 1,
    end: 2,
});

// TODO: handle disconnected graph
fn repeated_bfs(map: &Map) -> Output {
    let mut output = Vec::new();
    // let mut used_rooms_by_turn = Vec::new();
    for ant in 1..=map.ants {
        let mut queue = VecDeque::from([(0, map.start)]);
        let mut time_to_reach = vec![None; map.nodes.len()];
        let mut parents = vec![None; map.nodes.len()];

        time_to_reach[map.start] = Some(0);
        'outer: while let Some((time, node)) = queue.pop_front() {
            if node == map.end {
                break;
            }
            queue.push_back((time + 1, node));
            for &neighbor in &map.edges[node] {
                if parents[neighbor].is_some() {
                    continue;
                }
                parents[neighbor] = Some(node);
                time_to_reach[neighbor] = Some(time + 1);
                queue.push_back((time + 1, neighbor));
                if neighbor == map.end {
                    break 'outer;
                }
            }
        }

        println!("{:?}", parents);
    }

    output
}

fn print_result(result: &[Vec<Move>]) {
    todo!()
}

fn main() {
    let result = repeated_bfs(&MAP_SUBJECT_3);
    print_result(&result);
}
