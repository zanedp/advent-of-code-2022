const HEIGHT_MAX: char = 'z';

use std::{collections::VecDeque, str::FromStr, vec};

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
    elevations: HeightMap,
    start: Coordinate,
    end: Coordinate,
}

impl FromStr for Map {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height_map: HeightMap = s.lines().map(|line| Vec::from_iter(line.chars())).collect();

        let start = find_start(&height_map);
        if start.is_none() {
            return Err("could not find start, S");
        }

        let end = find_end(&height_map);
        if end.is_none() {
            return Err("could not find end, E");
        }

        Ok(Self {
            elevations: height_map,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Map {
    /// # Returns
    /// Tuple of (# rows, # columns)
    fn dimensions(&self) -> (usize, usize) {
        (self.elevations.len(), self.elevations[0].len())
    }

    /// Gets the neighboring squares of the specified position, but does not
    /// consider whether or not an explorer could actually move to that position.
    fn neighbor_squares(&self, pos: Coordinate) -> impl Iterator<Item = Coordinate> {
        let mut retval = Vec::new();
        let mut cur_dir = Up;
        while !cur_dir.is_none() {
            if let Some(new_pos) = pos.move_to_dir(cur_dir, &self.elevations) {
                retval.push(new_pos);
            }
            cur_dir = cur_dir.next();
        }
        Box::new(retval.into_iter())
    }

    /// Gets the height of the specified position.
    fn height(&self, coord: Coordinate) -> Height {
        self.elevations[coord.0][coord.1]
    }
}

fn find_distance(
    map: &Map,
    start: Coordinate,
    is_end_fn: impl Fn(Coordinate) -> bool,
    can_travel_to_fn: impl Fn(Height, Height) -> bool,
) -> Option<i32> {
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let dims = map.dimensions();
    let mut distances = vec![vec![None; dims.1]; dims.0];
    distances[start.0][start.1] = Some(0);
    queue.push_back(start);

    while !queue.is_empty() {
        let cur_pos = queue.pop_front().unwrap(); // ok because we checked for empty already
        let cur_distance = distances[cur_pos.0][cur_pos.1].unwrap();
        let neighbors = map.neighbor_squares(cur_pos);
        for n in neighbors {
            if distances[n.0][n.1].is_some() {
                // we already visited it
                continue;
            } else {
                // this neighbor needs to be explored
                let n_distance = cur_distance + 1;
                if can_travel_to_fn(map.height(cur_pos), map.height(n)) {
                    // and we can explore it now, since we can climb up or down to it from current position
                    if is_end_fn(n) {
                        return Some(n_distance);
                    }
                    distances[n.0][n.1] = Some(n_distance);
                    queue.push_back(n);
                }
            }
        }
    }
    None
}

fn find_distance_to_end(map: &Map) -> Option<i32> {
    find_distance(map, map.start, |n| n == map.end, can_climb_to)
}

fn find_distance_from_end_to_lowland(map: &Map) -> Option<i32> {
    find_distance(
        map,
        map.end,
        |n| map.elevations[n.0][n.1] == 'a',
        |from, to| can_climb_to(to, from),
    )
}

#[test]
fn test_dir_next() {
    assert_eq!(Up.next(), Right);
    assert_eq!(Right.next(), Down);
    assert_eq!(Down.next(), Left);
    assert_eq!(Left.next(), Direction::None);
    assert_eq!(Direction::None.next(), Direction::None);
}

/// Indicates whether a hiker can climb from a square of specified height to a square of another specified height.
///
/// A hiker can climb to another square:
/// - of the same elevation
/// - of lower elevation
/// - with an elevation one higher than where they currently are
///
/// A Height of 'S' is considered equivalent to height 'a' and a Height of 'E' is considered to be of height HEIGHT_MAX.
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
    let map = Map::from_str(input).unwrap();
    let distance = find_distance_to_end(&map);
    let distance_to_lowland = find_distance_from_end_to_lowland(&map);
    println!("distance = {:?}", distance);
    println!("distance to lowland = {:?}", distance_to_lowland);
}
