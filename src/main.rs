use itertools::Itertools as _;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead as _,
    iter::zip,
    sync::LazyLock,
};

type Path = Vec<usize>;
type Edge = (usize, usize);

#[derive(Clone, Debug)]
struct Node {
    name: String,
    // TODO: position for visualizer
    x: i64,
    y: i64,
}

// TODO: better constructor names
impl Node {
    fn new(name: impl Into<String>, x: i64, y: i64) -> Self {
        Self { name: name.into(), x, y }
    }

    fn origin(name: impl Into<String>) -> Self {
        Self { name: name.into(), x: 0, y: 0 }
    }
}

// TODO: remove Debug and impl a clean Display
#[derive(Debug)]
enum ParseMapError {
    IoError(std::io::Error),
    InvalidAntsNumber(String),
    InvalidNodeLine(String),
    InvalidCharacterInNodeName(String, char),
    NodeNameStartsWithL(String),
    DuplicateNodeName(String, String),
    InvalidNodeCoordinate(String, char, String),
    InvalidTag(String),
    InvalidEdgeLine(String),
    UnknownNodeNameInEdge(String, String),
    MultipleStartNodes,
    MultipleEndNodes,
    MissingAntsNumber,
    MissingNodes,
    MissingStartNode,
    MissingEndNode,
    MissingEdges,
}

impl From<std::io::Error> for ParseMapError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

#[derive(Clone, Debug)]
struct Map {
    ants: u32, // TODO: check not 0
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    start: usize,
    end: usize,
}

impl Map {
    fn parse() -> Result<Self, ParseMapError> {
        enum SpecialNode {
            Start,
            End,
        }

        enum ParsingState {
            Ants,
            Nodes,
            SpecialNode(SpecialNode),
            Edges,
        }

        let mut parsing_state = ParsingState::Ants;

        let mut node_indices = HashMap::new();

        let mut ants = None;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut start = None;
        let mut end = None;

        let stdin = std::io::stdin();
        for res_line in stdin.lock().lines() {
            let full_line = res_line?;
            let line = full_line.trim();

            if line.starts_with('#') && !line.starts_with("##") {
                continue;
            }

            match parsing_state {
                ParsingState::Ants => {
                    ants = Some(Self::parse_ants(line)?);
                    parsing_state = ParsingState::Nodes;
                }
                ParsingState::Nodes => match line {
                    "##start" => {
                        if start.is_some() {
                            return Err(ParseMapError::MultipleStartNodes);
                        }
                        parsing_state = ParsingState::SpecialNode(SpecialNode::Start);
                    }
                    "##end" => {
                        if end.is_some() {
                            return Err(ParseMapError::MultipleEndNodes);
                        }
                        parsing_state = ParsingState::SpecialNode(SpecialNode::End);
                    }
                    line if line.starts_with("##") => {
                        return Err(ParseMapError::InvalidTag(line.into()));
                    }
                    _ => match Self::parse_node(line, &node_indices) {
                        Ok(node) => {
                            node_indices.insert(node.name.clone(), nodes.len());
                            nodes.push(node);
                        }
                        Err(err_parse_node) => match Self::parse_edge(line, &node_indices) {
                            Ok((node1, node2)) => {
                                edges = vec![vec![]; nodes.len()];
                                edges[node1].push(node2);
                                edges[node2].push(node1);
                            }
                            Err(ParseMapError::InvalidEdgeLine(_)) => return Err(err_parse_node),
                            Err(err_parse_edge) => return Err(err_parse_edge),
                        },
                    },
                },
                ParsingState::SpecialNode(special_node) => {
                    let node = Self::parse_node(line, &node_indices)?;
                    let special_node_idx = Some(nodes.len());
                    match special_node {
                        SpecialNode::Start => start = special_node_idx,
                        SpecialNode::End => end = special_node_idx,
                    }
                    node_indices.insert(node.name.clone(), nodes.len());
                    nodes.push(node);
                    parsing_state = ParsingState::Nodes;
                }
                ParsingState::Edges => {
                    let (node1, node2) = Self::parse_edge(line, &node_indices)?;
                    edges[node1].push(node2);
                    edges[node2].push(node1);
                }
            }
        }

        let Some(ants) = ants else {
            return Err(ParseMapError::MissingAntsNumber);
        };
        if nodes.is_empty() {
            return Err(ParseMapError::MissingNodes);
        }
        let Some(start) = start else {
            return Err(ParseMapError::MissingStartNode);
        };
        let Some(end) = end else {
            return Err(ParseMapError::MissingEndNode);
        };
        if edges.is_empty() {
            return Err(ParseMapError::MissingEdges);
        }

        for neighbors in &mut edges {
            neighbors.sort_unstable();
        }

        Ok(Self { ants, nodes, edges, start, end })
    }

    fn parse_ants(line: &str) -> Result<u32, ParseMapError> {
        line.parse::<u32>().map_err(|_err| ParseMapError::InvalidAntsNumber(line.into()))
    }

    // TODO: '-' forbidden in node name or starting with 'L'
    fn parse_node(
        line: &str,
        node_indices: &HashMap<String, usize>,
    ) -> Result<Node, ParseMapError> {
        let parts = line.split_whitespace().collect_vec();
        let &[name, coord_x, coord_y] = parts.as_slice() else {
            return Err(ParseMapError::InvalidNodeLine(line.into()));
        };

        if let Some(invalid_char) = name.chars().find(|&c| !c.is_ascii_alphanumeric() && c != '_') {
            return Err(ParseMapError::InvalidCharacterInNodeName(line.into(), invalid_char));
        }
        if name.starts_with('L') {
            return Err(ParseMapError::NodeNameStartsWithL(line.into()));
        }
        if node_indices.contains_key(name) {
            return Err(ParseMapError::DuplicateNodeName(line.into(), name.into()));
        }
        let Ok(coord_x) = coord_x.parse() else {
            return Err(ParseMapError::InvalidNodeCoordinate(line.into(), 'x', coord_x.into()));
        };
        let Ok(coord_y) = coord_y.parse() else {
            return Err(ParseMapError::InvalidNodeCoordinate(line.into(), 'y', coord_y.into()));
        };

        Ok(Node::new(name, coord_x, coord_y))
    }

    fn parse_edge(
        line: &str,
        node_indices: &HashMap<String, usize>,
    ) -> Result<Edge, ParseMapError> {
        let parts = line.split('-').collect_vec();
        let &[node1, node2] = parts.as_slice() else {
            return Err(ParseMapError::InvalidEdgeLine(line.into()));
        };

        let Some(&idx1) = node_indices.get(node1) else {
            return Err(ParseMapError::UnknownNodeNameInEdge(line.into(), node1.into()));
        };
        let Some(&idx2) = node_indices.get(node2) else {
            return Err(ParseMapError::UnknownNodeNameInEdge(line.into(), node2.into()));
        };

        Ok((idx1, idx2))
    }
}

static MAP_SUBJECT_1: LazyLock<Map> = LazyLock::new(|| Map {
    ants: 3,
    nodes: vec![Node::origin("0"), Node::origin("1"), Node::origin("2"), Node::origin("3")],
    edges: vec![vec![2], vec![3], vec![3], vec![1, 2]],
    start: 0,
    end: 1,
});

static MAP_SUBJECT_2_2: LazyLock<Map> = LazyLock::new(|| Map {
    ants: 2,
    nodes: vec![
        Node::origin("1"),
        Node::origin("0"),
        Node::origin("4"),
        Node::origin("2"),
        Node::origin("3"),
    ],
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
        Node::origin("3"),
        Node::origin("start"),
        Node::origin("end"),
        Node::origin("4"),
        Node::origin("1"),
        Node::origin("2"),
        Node::origin("5"),
        Node::origin("6"),
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

/// Exponential but provably optimal solver to compare against faster solvers.
fn iterative_deepening_search(map: &Map) -> Vec<Path> {
    fn valid_next_nodes(next_nodes: &[usize], map: &Map) -> bool {
        let mut seen = HashSet::new();

        for next_node in next_nodes {
            if seen.contains(next_node) {
                return false;
            }
            if *next_node != map.end && *next_node != map.start {
                seen.insert(next_node);
            }
        }

        true
    }

    fn dfs(map: &Map, paths: &mut [Path], depth: u32) -> bool {
        if depth == 0 {
            return paths.iter().all(|path| path[path.len() - 1] == map.end);
        }

        let (remaining_ants, remaining_nodes): (Vec<_>, Vec<_>) = paths
            .iter()
            .enumerate()
            .map(|(ant, path)| (ant, path[path.len() - 1]))
            .filter(|(_, node)| *node != map.end)
            .unzip();

        for next_nodes in remaining_nodes
            .iter()
            .map(|&node| {
                let mut next_nodes = map.edges[node].clone();
                next_nodes.push(node);
                next_nodes
            })
            .multi_cartesian_product()
            .filter(|next_nodes| valid_next_nodes(next_nodes, map))
        {
            for (&ant, next_node) in zip(&remaining_ants, next_nodes) {
                paths[ant].push(next_node);
            }

            if dfs(map, paths, depth - 1) {
                return true;
            }

            for &ant in &remaining_ants {
                paths[ant].pop();
            }
        }

        false
    }

    for max_depth in 1.. {
        let mut paths = vec![vec![map.start]; map.ants as usize];
        if dfs(map, &mut paths, max_depth) {
            return paths;
        }
    }

    unreachable!()
}

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
    let map = Map::parse();
    println!("{map:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    type Solver = fn(&Map) -> Vec<Path>;

    fn solves(map: &Map, paths: &[Path]) -> bool {
        let max_path = paths.iter().map(Vec::len).max().unwrap();
        let mut ants = vec![map.start; map.ants as usize];
        for time in 1..max_path {
            let mut seen = HashSet::new();
            for (ant, path) in paths.iter().enumerate() {
                let Some(&node) = path.get(time) else {
                    continue;
                };
                if seen.contains(&node) {
                    return false;
                }
                if node != ants[ant] && !map.edges[ants[ant]].contains(&node) {
                    return false;
                }
                if node != map.end && node != map.start {
                    seen.insert(node);
                }
                ants[ant] = node;
            }
        }
        true
    }

    fn check_map_with_solver(solver: Solver, map: &Map, expected_time: usize) {
        let paths = solver(map);
        assert_eq!(paths.len(), map.ants as usize);
        assert!(solves(map, &paths));
        let max_path = paths.iter().map(Vec::len).max().unwrap();
        assert_eq!(max_path - 1, expected_time);
    }

    #[test]
    fn subject_1() {
        check_map_with_solver(repeated_bfs, &MAP_SUBJECT_1, 5);
        check_map_with_solver(iterative_deepening_search, &MAP_SUBJECT_1, 5);
    }

    #[test]
    fn subject_2_2() {
        check_map_with_solver(repeated_bfs, &MAP_SUBJECT_2_2, 3);
        check_map_with_solver(iterative_deepening_search, &MAP_SUBJECT_2_2, 3);
    }

    #[test]
    fn subject_2_3() {
        check_map_with_solver(repeated_bfs, &MAP_SUBJECT_2_3, 3);
        check_map_with_solver(iterative_deepening_search, &MAP_SUBJECT_2_3, 3);
    }

    #[test]
    fn subject_3() {
        check_map_with_solver(repeated_bfs, &MAP_SUBJECT_3, 5);
        check_map_with_solver(iterative_deepening_search, &MAP_SUBJECT_3, 5);
    }
}
