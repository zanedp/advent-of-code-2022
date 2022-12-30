const HEIGHT_MAX: char = 'z';

use std::{collections::VecDeque, vec};

type Height = char;
type HeightMap = Vec<Vec<Height>>;
use Direction::*;
use Option::None;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coordinate(usize, usize);
impl Coordinate {
    fn move_to_dir(self, dir: Direction, map: &HeightMap) -> Option<Self> {
        let last_row = map.len() - 1;
        let last_col = map[0].len() - 1;
        let new_coord = match dir {
            Up => Self(1.max(self.0) - 1, self.1),
            Right => Self(self.0, last_col.min(self.1 + 1)),
            Down => Self(last_row.min(self.0 + 1), self.1),
            Left => Self(self.0, 1.max(self.1) - 1),
            Direction::None => self,
        };
        if new_coord == self {
            None
        } else {
            Some(new_coord)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl Direction {
    fn is_none(self) -> bool {
        self == Self::None
    }

    fn next(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left | Self::None => Self::None,
        }
    }
}

#[derive(Debug)]
struct Map {
    elevation: HeightMap,
    start: Coordinate,
    end: Coordinate,
}

impl Map {
    fn new(height_map: Vec<Vec<Height>>) -> Self {
        let start = find_start(&height_map).expect("height_map should contain start, S");
        let end = find_end(&height_map).expect("height_map should contain end, E");
        Self {
            elevation: height_map,
            start,
            end,
        }
    }

    /// # Returns
    /// Tuple of (# rows, # columns)
    fn dimensions(&self) -> (usize, usize) {
        (self.elevation.len(), self.elevation[0].len())
    }

    /// Gets the neighboring squares of the specified position, but does not
    /// consider whether or not an explorer could actually move to that position.
    fn neighbor_squares(&self, pos: Coordinate) -> Box<dyn Iterator<Item = Coordinate>> {
        let mut retval = Vec::new();
        let mut cur_dir = Up;
        while !cur_dir.is_none() {
            if let Some(new_pos) = pos.move_to_dir(cur_dir, &self.elevation) {
                retval.push(new_pos);
            }
            cur_dir = cur_dir.next();
        }
        Box::new(retval.into_iter())
    }

    /// Gets the height of the specified position.
    fn height(&self, coord: Coordinate) -> Height {
        self.elevation[coord.0][coord.1]
    }

    /// Indicates whether the specified position is the end point.
    fn is_end(&self, coord: Coordinate) -> bool {
        self.height(coord) == 'E'
    }
}

#[derive(Debug)]
struct Explorer<'a> {
    queue: VecDeque<Coordinate>,
    distances: Vec<Vec<Option<i32>>>,
    map: &'a Map,
}

impl<'a> Explorer<'a> {
    fn new(map: &'a Map) -> Self {
        let dims = map.dimensions();
        Self {
            queue: VecDeque::new(),
            distances: vec![vec![None; dims.1]; dims.0],
            map,
        }
    }

    fn find_distance_to_end(&mut self) -> Option<i32> {
        let start = self.map.start;
        self.queue.push_back(start);
        self.distances[start.0][start.1] = Some(0);

        while !self.queue.is_empty() {
            let cur_pos = self.queue.pop_front().unwrap(); // ok because we checked for empty already
            let cur_distance = self.distances[cur_pos.0][cur_pos.1].unwrap();
            let neighbors = self.map.neighbor_squares(cur_pos);
            for n in neighbors {
                if self.distances[n.0][n.1].is_some() {
                    // we already visited it
                    continue;
                } else {
                    // this neighbor needs to be explored
                    if can_climb_to(self.map.height(cur_pos), self.map.height(n)) {
                        // and we can explore it now, since we can climb up or down to it from current position
                        if self.map.is_end(n) {
                            return Some(cur_distance + 1);
                        }
                        self.distances[n.0][n.1] = Some(cur_distance + 1);
                        self.queue.push_back(n);
                    }
                }
            }
        }
        None
    }

    fn find_distance_from_end_to_lowland(&self) -> Option<i32> {
        let dim = self.map.dimensions();
        let mut queue = VecDeque::new();
        let mut distances: Vec<Vec<Option<i32>>> = vec![vec![None; dim.1]; dim.0];
        let cur = self.map.end;
        distances[cur.0][cur.1] = Some(0);
        queue.push_back(cur);
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            let cur_distance = distances[cur.0][cur.1].unwrap();

            for n in self.map.neighbor_squares(cur) {
                if distances[n.0][n.1].is_some() {
                    // don't need to visit it again
                    continue;
                } else {
                    if can_climb_to(self.map.height(n), self.map.height(cur)) {
                        let n_distance = cur_distance + 1;
                        if self.map.height(n) == 'a' {
                            // this neighbor is a low point, so we're done
                            return Some(n_distance);
                        } else {
                            distances[n.0][n.1] = Some(n_distance);
                            queue.push_back(n);
                        }
                    }
                }
            }
        }
        None
    }
}

#[test]
fn test_dir_next() {
    assert_eq!(Up.next(), Right);
    assert_eq!(Right.next(), Down);
    assert_eq!(Down.next(), Left);
    assert_eq!(Left.next(), Direction::None);
    assert_eq!(Direction::None.next(), Direction::None);
}

fn can_climb_to(from: Height, to: Height) -> bool {
    assert_ne!(from, 'E');
    let this = if from == 'S' { 'a' } else { from };
    let other = if to == 'S' { 'a' } else { to };
    let other = if other == 'E' { HEIGHT_MAX } else { other };
    (this as u32) + 1 >= (other as u32)
}

#[test]
fn test_can_climb_to() {
    assert!(can_climb_to('a', 'b'));
    assert!(can_climb_to('b', 'a'));
    assert!(!can_climb_to('a', 'c'));
    assert!(can_climb_to('S', 'a'));
    assert!(can_climb_to('S', 'b'));
    assert!(!can_climb_to('a', 'E'));
    assert!(can_climb_to('y', 'E'));
}

fn find_start(height_map: &[Vec<Height>]) -> Option<Coordinate> {
    for (r, row) in height_map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                return Some(Coordinate(r, c));
            }
        }
    }
    None
}

fn find_end(height_map: &[Vec<Height>]) -> Option<Coordinate> {
    for (r, row) in height_map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'E' {
                return Some(Coordinate(r, c));
            }
        }
    }
    None
}

fn main() {
    // let input = include_str!("sample_input.txt");
    let input = include_str!("input.txt");
    let height_map: HeightMap = input
        .lines()
        .map(|line| Vec::from_iter(line.chars()))
        .collect();
    let map = Map::new(height_map);

    let mut explorer = Explorer::new(&map);
    let distance = explorer.find_distance_to_end();
    let distance_to_lowland = explorer.find_distance_from_end_to_lowland();
    println!("distance = {:?}", distance);
    println!("distance to lowland = {:?}", distance_to_lowland);
}
