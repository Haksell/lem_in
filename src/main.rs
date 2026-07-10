use std::{
    collections::{HashSet, VecDeque},
    sync::LazyLock,
};

use itertools::Itertools as _;

type Edge = (Node, Node);
type Path = Vec<usize>;
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
    ants: 4,
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

#[expect(clippy::unwrap_used)]
fn reconstruct_path(parents: &[Option<(usize, usize)>], start: usize, end: usize) -> Path {
    let mut path = vec![end];
    let mut node = end;
    let mut time = 0;
    while node != start {
        let (previous_node, previous_time) = parents[node].unwrap();
        for _ in previous_time..time {
            path.push(node);
        }
        node = previous_node;
        time = previous_time;
    }
    for _ in 0..time {
        path.push(start);
    }
    path.reverse();
    path
}

fn bfs(map: &Map, used_rooms_by_time: &[HashSet<usize>]) -> Path {
    let mut parents = vec![None; map.nodes.len()];
    parents[map.start] = Some((map.start, 0));
    let mut queue = VecDeque::from([(map.start, 0)]);

    while let Some((node, time)) = queue.pop_front() {
        for &neighbor in &map.edges[node] {
            if parents[neighbor].is_some()
                || used_rooms_by_time.get(time + 1).is_some_and(|used| used.contains(&neighbor))
            {
                continue;
            }

            parents[neighbor] = Some((node, time + 1));
            if neighbor == map.end {
                return reconstruct_path(&parents, map.start, map.end);
            }
            queue.push_back((neighbor, time + 1));
        }
        queue.push_back((node, time + 1));
    }

    unreachable!("TODO: handle disconnected graph")
}

fn repeated_bfs(map: &Map) -> Output {
    let mut output = Vec::new();

    // for (node, neighbors) in map.edges.iter().enumerate() {
    //     println!("{}", map.nodes[node].name);
    //     for &neighbor in neighbors {
    //         println!("- {}", map.nodes[neighbor].name);
    //     }
    // }

    let mut used_rooms_by_time = vec![HashSet::new()];
    for ant in 1..=map.ants {
        let path = bfs(map, &used_rooms_by_time);
        println!("{path:?}");
        println!("ant #{ant}: {}", path.iter().map(|&node| &map.nodes[node].name).join(" "));
        for time in 1..path.len() - 1 {
            let node = path[time];
            if used_rooms_by_time.len() <= time {
                used_rooms_by_time.push(HashSet::from([node]));
            } else {
                used_rooms_by_time[time].insert(node);
            }
        }
        // dbg!(&used_rooms_by_time);
    }

    output
}

fn print_result(result: &[Vec<Move>]) {
    // TODO
}

fn main() {
    let result = repeated_bfs(&MAP_SUBJECT_3);
    print_result(&result);
}
