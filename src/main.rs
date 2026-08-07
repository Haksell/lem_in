use itertools::Itertools as _;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead as _,
};

type Path = Vec<usize>;
type Link = (usize, usize);

#[derive(Clone, Debug, PartialEq)]
struct Room {
    name: String,
    x: i64,
    y: i64,
}

// TODO: better constructor names
impl Room {
    fn new(name: impl Into<String>, x: i64, y: i64) -> Self {
        Self { name: name.into(), x, y }
    }
}

// TODO: remove Debug and impl a clean Display
#[derive(Debug)]
enum ParseMapError {
    IoError(std::io::Error),
    InvalidAntsNumber(String),
    InvalidRoomLine(String),
    InvalidCharacterInRoomName(String, char),
    RoomNameStartsWithL(String),
    DuplicateRoomName(String, String),
    InvalidRoomCoordinate(String, char, String),
    InvalidTag(String),
    InvalidLinkLine(String),
    UnknownRoomNameInLink(String, String),
    MultipleStartRooms,
    MultipleEndRooms,
    MissingAntsNumber,
    MissingRooms,
    MissingStartRoom,
    MissingEndRoom,
    MissingLinks,
}

impl From<std::io::Error> for ParseMapError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Map {
    ants: u32, // TODO: check not 0
    rooms: Vec<Room>,
    links: Vec<Vec<usize>>,
    start: usize,
    end: usize,
}

impl Map {
    fn parse() -> Result<Self, ParseMapError> {
        enum SpecialRoom {
            Start,
            End,
        }

        enum ParsingState {
            Ants,
            Rooms,
            SpecialRoom(SpecialRoom),
            Links,
        }

        let mut parsing_state = ParsingState::Ants;
        let mut room_indices = HashMap::new();

        let mut ants = None;
        let mut rooms = Vec::new();
        let mut links = Vec::new();
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
                    parsing_state = ParsingState::Rooms;
                }
                ParsingState::Rooms => match line {
                    "##start" => {
                        if start.is_some() {
                            return Err(ParseMapError::MultipleStartRooms);
                        }
                        parsing_state = ParsingState::SpecialRoom(SpecialRoom::Start);
                    }
                    "##end" => {
                        if end.is_some() {
                            return Err(ParseMapError::MultipleEndRooms);
                        }
                        parsing_state = ParsingState::SpecialRoom(SpecialRoom::End);
                    }
                    line if line.starts_with("##") => {
                        return Err(ParseMapError::InvalidTag(line.into()));
                    }
                    _ => match Self::parse_room(line, &room_indices) {
                        Ok(room) => {
                            room_indices.insert(room.name.clone(), rooms.len());
                            rooms.push(room);
                        }
                        Err(err_parse_room) => match Self::parse_link(line, &room_indices) {
                            Ok((room1, room2)) => {
                                links = vec![vec![]; rooms.len()];
                                links[room1].push(room2);
                                links[room2].push(room1);
                                parsing_state = ParsingState::Links;
                            }
                            Err(ParseMapError::InvalidLinkLine(_)) => return Err(err_parse_room),
                            Err(err_parse_link) => return Err(err_parse_link),
                        },
                    },
                },
                ParsingState::SpecialRoom(special_room) => {
                    let room = Self::parse_room(line, &room_indices)?;
                    let special_room_idx = Some(rooms.len());
                    match special_room {
                        SpecialRoom::Start => start = special_room_idx,
                        SpecialRoom::End => end = special_room_idx,
                    }
                    room_indices.insert(room.name.clone(), rooms.len());
                    rooms.push(room);
                    parsing_state = ParsingState::Rooms;
                }
                ParsingState::Links => {
                    let (room1, room2) = Self::parse_link(line, &room_indices)?;
                    links[room1].push(room2);
                    links[room2].push(room1);
                }
            }
        }

        let Some(ants) = ants else {
            return Err(ParseMapError::MissingAntsNumber);
        };
        if rooms.is_empty() {
            return Err(ParseMapError::MissingRooms);
        }
        let Some(start) = start else {
            return Err(ParseMapError::MissingStartRoom);
        };
        let Some(end) = end else {
            return Err(ParseMapError::MissingEndRoom);
        };
        if links.is_empty() {
            return Err(ParseMapError::MissingLinks);
        }

        for neighbors in &mut links {
            neighbors.sort_unstable();
        }

        Ok(Self { ants, rooms, links, start, end })
    }

    fn parse_ants(line: &str) -> Result<u32, ParseMapError> {
        line.parse::<u32>().map_err(|_err| ParseMapError::InvalidAntsNumber(line.into()))
    }

    fn parse_room(
        line: &str,
        room_indices: &HashMap<String, usize>,
    ) -> Result<Room, ParseMapError> {
        let parts = line.split_whitespace().collect_vec();
        let &[name, coord_x, coord_y] = parts.as_slice() else {
            return Err(ParseMapError::InvalidRoomLine(line.into()));
        };

        if let Some(invalid_char) = name.chars().find(|&c| !c.is_ascii_alphanumeric() && c != '_') {
            return Err(ParseMapError::InvalidCharacterInRoomName(line.into(), invalid_char));
        }
        if name.starts_with('L') {
            return Err(ParseMapError::RoomNameStartsWithL(line.into()));
        }
        if room_indices.contains_key(name) {
            return Err(ParseMapError::DuplicateRoomName(line.into(), name.into()));
        }
        let Ok(coord_x) = coord_x.parse() else {
            return Err(ParseMapError::InvalidRoomCoordinate(line.into(), 'x', coord_x.into()));
        };
        let Ok(coord_y) = coord_y.parse() else {
            return Err(ParseMapError::InvalidRoomCoordinate(line.into(), 'y', coord_y.into()));
        };

        Ok(Room::new(name, coord_x, coord_y))
    }

    fn parse_link(
        line: &str,
        room_indices: &HashMap<String, usize>,
    ) -> Result<Link, ParseMapError> {
        let parts = line.split('-').collect_vec();
        let &[room1, room2] = parts.as_slice() else {
            return Err(ParseMapError::InvalidLinkLine(line.into()));
        };

        let Some(&idx1) = room_indices.get(room1) else {
            return Err(ParseMapError::UnknownRoomNameInLink(line.into(), room1.into()));
        };
        let Some(&idx2) = room_indices.get(room2) else {
            return Err(ParseMapError::UnknownRoomNameInLink(line.into(), room2.into()));
        };

        Ok((idx1, idx2))
    }
}

fn reconstruct_path(parents: &[Option<(usize, usize)>], start: usize, end: usize) -> Path {
    let mut path = vec![end];
    let mut room = end;
    let mut time = 0;
    while room != start {
        let (previous_room, previous_time) = parents[room].unwrap();
        for _ in previous_time..time {
            path.push(room);
        }
        room = previous_room;
        time = previous_time;
    }
    for _ in 0..time {
        path.push(start);
    }
    path.reverse();
    path
}

fn bfs(map: &Map, used_rooms_by_time: &[HashSet<usize>]) -> Path {
    let mut parents = vec![None; map.rooms.len()];
    parents[map.start] = Some((map.start, 0));
    let mut queue = VecDeque::from([(map.start, 0)]);

    while let Some((room, time)) = queue.pop_front() {
        for &neighbor in &map.links[room] {
            if parents[neighbor].is_some()
                || used_rooms_by_time
                    .get(time + 1)
                    .is_some_and(|used_rooms| used_rooms.contains(&neighbor))
            {
                continue;
            }

            parents[neighbor] = Some((room, time + 1));
            if neighbor == map.end {
                return reconstruct_path(&parents, map.start, map.end);
            }
            queue.push_back((neighbor, time + 1));
        }
        queue.push_back((room, time + 1));
    }

    unreachable!("TODO: handle disconnected graph")
}

fn update_used_rooms(used_rooms_by_time: &mut Vec<HashSet<usize>>, path: &Path) {
    for time in 1..path.len() - 1 {
        while used_rooms_by_time.len() <= time {
            used_rooms_by_time.push(HashSet::new());
        }
        used_rooms_by_time[time].insert(path[time]);
    }
}

fn repeated_bfs(map: &Map) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut used_rooms_by_time = vec![];
    for _ in 0..map.ants {
        let path = bfs(map, &used_rooms_by_time);
        update_used_rooms(&mut used_rooms_by_time, &path);
        paths.push(path);
    }
    paths
}

fn print_moves(map: &Map, paths: &[Path]) {
    let max_time = paths.iter().map(Vec::len).max().unwrap();
    let mut previous = vec![map.start; paths.len()];
    for time in 1..max_time {
        let mut first_in_line = true;
        for (ant, path) in paths.iter().enumerate() {
            let Some(&room) = path.get(time) else {
                continue;
            };
            if room != previous[ant] {
                previous[ant] = room;
                print!(
                    "{}L{}-{}",
                    if first_in_line { "" } else { " " },
                    ant + 1,
                    map.rooms[room].name
                );
                first_in_line = false;
            }
        }
        println!();
    }
}

#[expect(clippy::print_stderr)]
fn main() {
    let map = Map::parse().unwrap_or_else(|err| {
        eprintln!("Error: {err:?}"); // TODO: Debug -> Display
        std::process::exit(1);
    });
    let paths = repeated_bfs(&map);
    print_moves(&map, &paths);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{iter::zip, sync::LazyLock};

    type Solver = fn(&Map) -> Vec<Path>;

    static MAP_SUBJECT_1: LazyLock<Map> = LazyLock::new(|| Map {
        ants: 3,
        rooms: vec![
            Room::new("0", 1, 0),
            Room::new("1", 5, 0),
            Room::new("2", 9, 0),
            Room::new("3", 13, 0),
        ],
        links: vec![vec![2], vec![3], vec![0, 3], vec![1, 2]],
        start: 0,
        end: 1,
    });

    static MAP_SUBJECT_2_2: LazyLock<Map> = LazyLock::new(|| Map {
        ants: 2,
        rooms: vec![
            Room::new("1", 0, 2),
            Room::new("0", 2, 0),
            Room::new("4", 2, 6),
            Room::new("2", 4, 2),
            Room::new("3", 4, 4),
        ],
        links: vec![vec![1, 2], vec![0, 3], vec![0, 4], vec![1, 4], vec![2, 3]],
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
        rooms: vec![
            Room::new("3", 2, 2),
            Room::new("start", 4, 0),
            Room::new("end", 4, 6),
            Room::new("4", 0, 4),
            Room::new("1", 4, 2),
            Room::new("2", 4, 4),
            Room::new("5", 8, 2),
            Room::new("6", 8, 4),
        ],
        links: vec![
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

    /// Exponential but provably optimal solver to compare against faster solvers.
    fn iterative_deepening_search(map: &Map) -> Vec<Path> {
        fn valid_next_rooms(next_rooms: &[usize], map: &Map) -> bool {
            let mut seen = HashSet::new();

            for next_room in next_rooms {
                if seen.contains(next_room) {
                    return false;
                }
                if *next_room != map.end && *next_room != map.start {
                    seen.insert(next_room);
                }
            }

            true
        }

        fn dfs(map: &Map, paths: &mut [Path], depth: u32) -> bool {
            if depth == 0 {
                return paths.iter().all(|path| path[path.len() - 1] == map.end);
            }

            let (remaining_ants, remaining_rooms): (Vec<_>, Vec<_>) = paths
                .iter()
                .enumerate()
                .map(|(ant, path)| (ant, path[path.len() - 1]))
                .filter(|(_, room)| *room != map.end)
                .unzip();

            for next_rooms in remaining_rooms
                .iter()
                .map(|&room| {
                    let mut next_rooms = map.links[room].clone();
                    next_rooms.push(room);
                    next_rooms
                })
                .multi_cartesian_product()
                .filter(|next_rooms| valid_next_rooms(next_rooms, map))
            {
                for (&ant, next_room) in zip(&remaining_ants, next_rooms) {
                    paths[ant].push(next_room);
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

    fn solves(map: &Map, paths: &[Path]) -> bool {
        let max_path = paths.iter().map(Vec::len).max().unwrap();
        let mut ants = vec![map.start; map.ants as usize];
        for time in 1..max_path {
            let mut seen = HashSet::new();
            for (ant, path) in paths.iter().enumerate() {
                let Some(&room) = path.get(time) else {
                    continue;
                };
                if seen.contains(&room) {
                    return false;
                }
                if room != ants[ant] && !map.links[ants[ant]].contains(&room) {
                    return false;
                }
                if room != map.end && room != map.start {
                    seen.insert(room);
                }
                ants[ant] = room;
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
