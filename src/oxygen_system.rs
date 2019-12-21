use super::intcode::Computer;
use itertools::Itertools;
use itertools::MinMaxResult;
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    WALL = 0,
    DISCOVERED = 1,
    SENSOR = 2,
    EXPLORED = 3,
    ROBOT = 4,
    UNKNOWN = 5,
}

impl From<i64> for Tile {
    fn from(x: i64) -> Tile {
        match x {
            0 => Tile::WALL,
            1 => Tile::DISCOVERED,
            2 => Tile::SENSOR,
            3 => Tile::EXPLORED,
            _ => panic!("tile not recognized: {}", x),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile_str = match self {
            Tile::WALL => "#",
            Tile::DISCOVERED => ".",
            Tile::SENSOR => "o",
            Tile::EXPLORED => "+",
            Tile::ROBOT => "@",
            Tile::UNKNOWN => " ",
        };
        write!(f, "{}", tile_str)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}

impl Direction {
    fn iter() -> std::vec::IntoIter<Direction> {
        vec![
            Direction::NORTH,
            Direction::SOUTH,
            Direction::WEST,
            Direction::EAST,
        ]
        .into_iter()
    }
    fn from_dxdy(dxdy: (isize, isize)) -> Direction {
        match dxdy {
            (0, -1) => Direction::NORTH,
            (0, 1) => Direction::SOUTH,
            (-1, 0) => Direction::WEST,
            (1, 0) => Direction::EAST,
            _ => panic!("these coordinates are not adjacent: {}, {}", dxdy.0, dxdy.1),
        }
    }
    fn dxdy(&self) -> (isize, isize) {
        match self {
            Direction::NORTH => (0, -1),
            Direction::SOUTH => (0, 1),
            Direction::WEST => (-1, 0),
            Direction::EAST => (1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Add<&Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Direction) -> Coordinate {
        let (dx, dy) = rhs.dxdy();
        Coordinate {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl AddAssign<&Direction> for Coordinate {
    fn add_assign(&mut self, rhs: &Direction) {
        let (dx, dy) = rhs.dxdy();
        self.x += dx;
        self.y += dy;
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Direction;
    fn sub(self, rhs: Coordinate) -> Self::Output {
        Direction::from_dxdy((rhs.x - self.x, rhs.y - self.y))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl Coordinate {
    fn distance(a: &Coordinate, b: &Coordinate) -> isize {
        (b.x - a.x).abs() + (b.y - a.y).abs()
    }
    fn priority(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

struct Robot {
    position: Coordinate,
    map: HashMap<Coordinate, Tile>,
    sensor: Option<Coordinate>,
    computer: Computer,
    debug_print: bool,
}

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            position: Coordinate::default(),
            map: {
                let mut map = HashMap::new();
                map.insert(Coordinate::default(), Tile::DISCOVERED);
                map
            },
            computer: Computer::new(Computer::load_data(15), &[]),
            sensor: None,
            debug_print: false,
        }
    }
}

impl Robot {
    fn print_map(&self) {
        let (x_min, x_max, y_min, y_max) = {
            let x_bounds = self.map.keys().minmax_by_key(|&v| v.x);
            let y_bounds = self.map.keys().minmax_by_key(|&v| v.y);
            match (x_bounds, y_bounds) {
                (MinMaxResult::MinMax(x_min, x_max), MinMaxResult::MinMax(y_min, y_max)) => {
                    (x_min.x - 1, x_max.x + 1, y_min.y - 1, y_max.y + 1)
                }
                _ => (-5, 5, -5, 5),
            }
        };
        println!("******************************");
        for y in y_min..y_max {
            for x in x_min..x_max {
                let tile = {
                    match (self.sensor, self.position) {
                        (Some(coord), _) if coord.x == x && coord.y == y => Tile::SENSOR,
                        (_, coord) if coord.x == x && coord.y == y => Tile::ROBOT,
                        _ => *self.map.get(&Coordinate { x, y }).unwrap_or(&Tile::UNKNOWN),
                    }
                };
                print!("{}", tile);
            }
            println!()
        }
        println!("******************************");
    }
    fn do_move(&mut self, direction: &Direction) -> bool {
        self.computer.input_queue.push_back(*direction as i64);
        let response = self.computer.next().map(Tile::from);
        match response {
            Some(Tile::DISCOVERED) => {
                self.position += direction;
                self.map.entry(self.position).or_insert(Tile::DISCOVERED);
                true
            }
            Some(Tile::SENSOR) => {
                self.position += direction;
                self.map.entry(self.position).or_insert(Tile::DISCOVERED);
                self.sensor = Some(self.position);
                true
            }
            Some(Tile::WALL) => {
                self.map
                    .entry(self.position + direction)
                    .or_insert(Tile::WALL);
                false
            }
            Some(Tile::EXPLORED) | Some(Tile::ROBOT) | Some(Tile::UNKNOWN) => panic!(
                "robot computer should never respond with this tile: {:?}",
                response
            ),
            None => {
                panic!("something terrible happened, got none response back");
            }
        }
    }
    fn find_path(&self, src: &Coordinate, target: &Coordinate) -> (Vec<Coordinate>, isize) {
        astar(
            src,
            |coord| {
                Direction::iter()
                    .map(|d| coord.clone() + &d)
                    .filter(|new_coord| match self.map.get(new_coord) {
                        Some(Tile::DISCOVERED) | Some(Tile::EXPLORED) => true,
                        _ => false,
                    })
                    .map(|coord| (coord, 1))
                    .collect::<Vec<(Coordinate, isize)>>()
            },
            |coord| Coordinate::distance(coord, target),
            |coord| coord == target,
        )
        .unwrap()
    }
    fn navigate_path(&mut self, path: Vec<Coordinate>) {
        assert_eq!(self.position, path[0]);
        for i in 1..path.len() {
            let direction = path[i] - path[i - 1];
            assert!(self.do_move(&direction));
            assert_eq!(self.position, path[i]);
        }
    }
    fn navigate_to(&mut self, target: &Coordinate) {
        let (path, _) = self.find_path(&self.position, target);
        self.navigate_path(path);
    }
    fn explore(&mut self) {
        loop {
            let mut nearby_unexplored =
                Direction::iter().filter(|z| !self.map.contains_key(&(self.position + z)));

            match nearby_unexplored.next() {
                Some(direction) => {
                    self.do_move(&direction);
                    continue;
                }
                None => {
                    self.map.insert(self.position, Tile::EXPLORED);
                }
            }

            let mut nearby_partially_explored =
                Direction::iter().filter(|z| match self.map.get(&(self.position + z)) {
                    Some(Tile::DISCOVERED) => true,
                    _ => false,
                });

            match nearby_partially_explored.next() {
                Some(direction) => {
                    self.do_move(&direction);
                    continue;
                }
                None => (),
            }

            let mut possible_unexplored = self
                .map
                .iter()
                .filter(|&(_, &value)| value == Tile::DISCOVERED)
                .map(|(&key, _)| key);

            match possible_unexplored.next() {
                Some(coord) => self.navigate_to(&coord),
                None => {
                    if self.debug_print {
                        self.print_map();
                    }
                    break;
                }
            }
        }
    }
}

fn flood(map: &HashMap<Coordinate, Tile>, source: &Coordinate) -> usize {
    let mut oxygenated: HashSet<Coordinate> = HashSet::new();
    oxygenated.insert(*source);

    let mut frontier: HashSet<Coordinate> = HashSet::new();
    frontier.insert(*source);

    let mut i = 0;

    loop {
        let new_coordinates: HashSet<Coordinate> = frontier
            .iter()
            .flat_map(|&coord| {
                Direction::iter()
                    .map(|d| coord + &d)
                    .collect::<Vec<Coordinate>>()
            })
            .filter(|coord| !oxygenated.contains(coord))
            .filter(|coord| match map.get(coord) {
                Some(Tile::EXPLORED) | Some(Tile::DISCOVERED) => true,
                _ => false,
            })
            .collect();

        if new_coordinates.is_empty() {
            break;
        }
        frontier.clear();
        for coord in new_coordinates {
            oxygenated.insert(coord);
            frontier.insert(coord);
        }
        i += 1;
    }
    i
}

pub fn solve_part_one() -> isize {
    let mut robot = Robot::default();
    robot.debug_print = false;
    robot.explore();
    assert!(robot.sensor.is_some());
    let (_, cost) = robot.find_path(&Coordinate { x: 0, y: 0 }, &robot.sensor.unwrap());
    cost
}

pub fn solve_part_two() -> usize {
    let mut robot = Robot::default();
    robot.debug_print = false;
    robot.explore();
    assert!(robot.sensor.is_some());
    flood(&robot.map, &robot.sensor.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wtf() {
        let direction = &Direction::NORTH;
        assert_eq!(*direction as i64, 1);
    }

    #[test]
    fn test_robot_explore() {
        let mut robot = Robot::default();
        robot.explore();
        assert!(robot
            .map
            .values()
            .all(|&tile| tile == Tile::EXPLORED || tile == Tile::WALL));
    }

    #[test]
    fn test_flood() {
        let map: HashMap<Coordinate, Tile> = vec![
            ((1, 0), Tile::WALL),
            ((2, 0), Tile::WALL),
            ((0, 1), Tile::WALL),
            ((1, 1), Tile::EXPLORED),
            ((2, 1), Tile::EXPLORED),
            ((3, 1), Tile::WALL),
            ((4, 1), Tile::WALL),
            ((0, 2), Tile::WALL),
            ((1, 2), Tile::EXPLORED),
            ((2, 2), Tile::WALL),
            ((3, 2), Tile::EXPLORED),
            ((4, 2), Tile::EXPLORED),
            ((5, 2), Tile::WALL),
            ((0, 3), Tile::WALL),
            ((1, 3), Tile::EXPLORED),
            ((2, 3), Tile::EXPLORED),
            ((3, 3), Tile::EXPLORED),
            ((4, 3), Tile::WALL),
            ((1, 4), Tile::WALL),
            ((2, 4), Tile::WALL),
            ((3, 4), Tile::WALL),
        ]
        .into_iter()
        .map(|((x, y), v)| (Coordinate { x, y }, v))
        .collect();

        assert_eq!(flood(&map, &Coordinate { x: 2, y: 3 }), 4);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 330);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 352);
    }
}
