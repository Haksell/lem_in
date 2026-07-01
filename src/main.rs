use std::{io::BufRead as _, str::FromStr};

#[derive(Debug)]
enum LemInError {
    IoError(std::io::Error),
    EmptyFile,
    InvalidAnts(String),
    InvalidStartNode(String),
    InvalidEndNode(String),
    InvalidLine(String),
    InvalidEdge(String),
    MissingStartNode,
    MissingEndNode,
    MissingStartAndEndNodes,
    MultipleStartNodes,
    MultipleEndNodes,
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

impl FromStr for Node {
    type Err = LemInError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.split_whitespace().collect();

        Ok()
    }
}

struct Map {
    ants: u64,
    start: Node,
    end: Node,
    nodes: Vec<Node>,
    edges: Vec<(Node, Node)>,
}

impl Map {
    fn parse() -> Result<Self, LemInError> {
        enum ParsingState {
            Ants,
            Nodes(u64, Option<Node>, Option<Node>, Vec<Node>),
            StartNode(u64, Option<Node>, Vec<Node>),
            EndNode(u64, Option<Node>, Vec<Node>),
            Edges(u64, Option<Node>, Option<Node>, Vec<Node>,Vec<Edge>),
        }

        let mut state = ParsingState::Ants;

        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line?.trim().to_string();
            if line.is_empty() {
                continue;
            }

            match state {
                ParsingState::Ants => {
                    let Ok(ants) = line.parse() else {
                        return Err(LemInError::InvalidAnts(line));
                    };
                    state = ParsingState::Nodes(ants, None, None, Vec::new());
                }
                ParsingState::Nodes(_, start_node, end_node, nodes) => ,
                ParsingState::StartNode(_, end_node, nodes) => todo!(),
                ParsingState::EndNode(_, start_node, nodes) => todo!(),
                ParsingState::Edges(map) => todo!(),
            }
        }

        match state {
            ParsingState::Ants => return Err(LemInError::EmptyFile),
            ParsingState::Nodes(_, start_node, end_node, nodes) => todo!(),
            ParsingState::StartNode(_, node, nodes) => return Err(LemInError::MissingStartNode),
            ParsingState::EndNode(_, node, nodes) => return Err(LemInError::MissingEndNode),
            ParsingState::Edges(map) => todo!(),
        }

        Ok(Self { ants, start, end, nodes, edges })
    }
}

fn main() {
    let mut map = Map::parse();
}
