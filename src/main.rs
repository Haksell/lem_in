use std::{
    collections::{HashSet, VecDeque},
    sync::LazyLock,
};

type Path = Vec<usize>;

#[derive(Clone)]
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

#[derive(Clone)]
struct Map {
    ants: u32, // TODO: check not 0
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

static MAP_SUBJECT_2_2: LazyLock<Map> = LazyLock::new(|| Map {
    ants: 2,
    nodes: vec![Node::new("1"), Node::new("0"), Node::new("4"), Node::new("2"), Node::new("3")],
    edges: vec![vec![1, 2], vec![0, 3], vec![0, 4], vec![1, 4], vec![2, 3]],
    start: 1,
    end: 2,
});

static MAP_SUBJECT_2_3: LazyLock<Map> = LazyLock::new(|| {
    let mut map = MAP_SUBJECT_2_2.clone();
    map.ants = 3;
    map
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

fn bfs(map: &Map, used_nodes_by_time: &[HashSet<usize>]) -> Path {
    let mut parents = vec![None; map.nodes.len()];
    parents[map.start] = Some((map.start, 0));
    let mut queue = VecDeque::from([(map.start, 0)]);

    while let Some((node, time)) = queue.pop_front() {
        for &neighbor in &map.edges[node] {
            if parents[neighbor].is_some()
                || used_nodes_by_time
                    .get(time + 1)
                    .is_some_and(|used_nodes| used_nodes.contains(&neighbor))
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

fn update_used_nodes(used_nodes_by_time: &mut Vec<HashSet<usize>>, path: &Path) {
    for time in 1..path.len() - 1 {
        while used_nodes_by_time.len() <= time {
            used_nodes_by_time.push(HashSet::new());
        }
        used_nodes_by_time[time].insert(path[time]);
    }
}

fn repeated_bfs(map: &Map) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut used_nodes_by_time = vec![];
    for _ in 0..map.ants {
        let path = bfs(map, &used_nodes_by_time);
        update_used_nodes(&mut used_nodes_by_time, &path);
        paths.push(path);
    }
    paths
}

#[expect(clippy::print_stdout)]
fn print_moves(map: &Map, paths: &[Path]) {
    let max_time = paths.iter().map(Vec::len).max().unwrap();
    let mut previous = vec![map.start; paths.len()];
    for time in 1..max_time {
        let mut first_in_line = true;
        for (ant, path) in paths.iter().enumerate() {
            let Some(&node) = path.get(time) else {
                continue;
            };
            if node != previous[ant] {
                previous[ant] = node;
                print!(
                    "{}L{}-{}",
                    if first_in_line { "" } else { " " },
                    ant + 1,
                    map.nodes[node].name
                );
                first_in_line = false;
            }
        }
        println!();
    }
}

fn main() {
    let map = &MAP_SUBJECT_2_3;
    let paths = repeated_bfs(map);
    print_moves(map, &paths);
}
