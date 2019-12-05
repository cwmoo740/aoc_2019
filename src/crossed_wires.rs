extern crate regex;

use std::fmt;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::Add;

use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    R,
    D,
    L,
    U,
}

#[derive(Debug, Eq, PartialEq)]
struct Move {
    direction: Direction,
    magnitude: isize,
}


impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<dir>[URDL])(?P<mag>\d+)").unwrap();
        }
        for caps in RE.captures_iter(s) {
            let dir = match &caps["dir"] {
                "R" => Direction::R,
                "D" => Direction::D,
                "L" => Direction::L,
                "U" => Direction::U,
                _ => panic!("something terrible happened!"),
            };
            let mag = isize::from_str(&caps["mag"]).unwrap();
            return Ok(
                Move {
                    magnitude: mag,
                    direction: dir,
                }
            );
        }
        panic!("never found a match: {}", s);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
}

static ZERO_COORDINATE: Coordinate = Coordinate { x: 0, y: 0 };

impl Add<&Move> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Move) -> Coordinate {
        match rhs.direction {
            Direction::L => Coordinate { x: self.x - rhs.magnitude, y: self.y },
            Direction::U => Coordinate { x: self.x, y: self.y + rhs.magnitude },
            Direction::R => Coordinate { x: self.x + rhs.magnitude, y: self.y },
            Direction::D => Coordinate { x: self.x, y: self.y - rhs.magnitude },
        }
    }
}

fn parse_path(input: &str) -> Vec<Coordinate> {
    input
        .split(",")
        .map(Move::from_str)
        .collect::<Result<Vec<Move>, _>>()
        .unwrap()
        .into_iter()
        .fold(
            vec![Coordinate { x: 0, y: 0 }],
            |mut acc: Vec<Coordinate>, next_move: Move| {
                let next_coord = acc.last().unwrap() + &next_move;
                acc.push(next_coord);
                acc
            },
        )
}

fn parse_input(input: String) -> (Vec<Coordinate>, Vec<Coordinate>) {
    let mut moves = input
        .split("\n")
        .map(parse_path)
        .collect::<Vec<Vec<Coordinate>>>()
        .into_iter();

    (moves.next().unwrap(), moves.next().unwrap())
}

fn overlapping_coordinate(p0: &Coordinate, p1: &Coordinate, p2: &Coordinate, p3: &Coordinate) -> Option<Coordinate> {
    let p0_x = p0.x as f64;
    let p0_y = p0.y as f64;

    let p1_x = p1.x as f64;
    let p1_y = p1.y as f64;

    let p2_x = p2.x as f64;
    let p2_y = p2.y as f64;

    let p3_x = p3.x as f64;
    let p3_y = p3.y as f64;

    let s1_x = p1_x - p0_x;
    let s1_y = p1_y - p0_y;
    let s2_x = p3_x - p2_x;
    let s2_y = p3_y - p2_y;

    let det = -s2_x * s1_y + s1_x * s2_y;
    let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / det;
    let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / det;

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        let x = (p0_x + (t * s1_x)) as isize;
        let y = (p0_y + (t * s1_y)) as isize;
        Some(Coordinate { x, y })
    } else {
        None
    }
}

fn manhattan_distance(x: &Coordinate, y: &Coordinate) -> usize {
    ((x.x - y.x).abs() + (x.y - y.y).abs()) as usize
}

fn find_closest_intersection_by_manhattan_distance(wire0: &Vec<Coordinate>, wire1: &Vec<Coordinate>) -> Option<Coordinate> {
    let mut intersection_points: Vec<Coordinate> = vec![];
    for (x0, y0) in wire0.iter().zip(wire0[1..].iter()) {
        for (x1, y1) in wire1.iter().zip(wire1[1..].iter()) {
            if let Some(overlap) = overlapping_coordinate(x0, y0, x1, y1) {
                if overlap != ZERO_COORDINATE {
                    intersection_points.push(overlap);
                }
            }
        }
    }
    if intersection_points.is_empty() {
        None
    } else {
        let min_point = intersection_points
            .iter()
            .min_by(|&p0, &p1| {
                let m1 = manhattan_distance(p0, &ZERO_COORDINATE);
                let m2 = manhattan_distance(p1, &ZERO_COORDINATE);
                m1.cmp(&m2)
            })
            .unwrap();
        Some(
            Coordinate {
                x: min_point.x,
                y: min_point.y,
            }
        )
    }
}

pub fn solve_part_one() -> Option<usize> {
    let input = super::get_input::main(3);
    let (wire0, wire1) = parse_input(input);
    if let Some(coord) = find_closest_intersection_by_manhattan_distance(&wire0, &wire1) {
        Some(manhattan_distance(&coord, &Coordinate { x: 0, y: 0 }))
    } else {
        None
    }
}

fn find_closest_intersection_by_wire_length(wire0: &Vec<Coordinate>, wire1: &Vec<Coordinate>) -> Option<(Coordinate, usize)> {
    let mut d0: usize = 0;
    let mut d1: usize = 0;
    let mut intersection_points: Vec<(Coordinate, usize)> = vec![];
    for (x0, y0) in wire0.iter().zip(wire0[1..].iter()) {
        d0 += manhattan_distance(x0, y0);
        d1 = 0;
        for (x1, y1) in wire1.iter().zip(wire1[1..].iter()) {
            d1 += manhattan_distance(x1, y1);
            if let Some(overlap) = overlapping_coordinate(x0, y0, x1, y1) {
                if overlap != ZERO_COORDINATE {
                    let distance = d0 + d1 - manhattan_distance(y0, &overlap) - manhattan_distance(y1, &overlap);
                    intersection_points.push((overlap, distance));
                }
            }
        }
    }
    if intersection_points.is_empty() {
        None
    } else {
        let (point, distance) = intersection_points
            .iter()
            .min_by_key(|(_, distance)| *distance)
            .unwrap();
        Some(
            (
                Coordinate { x: point.x, y: point.y },
                *distance
            )
        )
    }
}

pub fn solve_part_two() -> Option<usize> {
    let input = super::get_input::main(3);
    let (wire0, wire1) = parse_input(input);
    if let Some((coord, distance)) = find_closest_intersection_by_wire_length(&wire0, &wire1) {
        Some(distance)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_from_str() {
        let values: Vec<(&str, Move)> = vec![
            ("R7", Move { direction: Direction::R, magnitude: 7 }),
            ("D273", Move { direction: Direction::D, magnitude: 273 }),
            ("L21", Move { direction: Direction::L, magnitude: 21 }),
            ("U3", Move { direction: Direction::U, magnitude: 3 }),
        ];
        for (x, y) in values {
            assert_eq!(Move::from_str(x).unwrap(), y);
        }
    }

    #[test]
    fn test_parse_path() {
        let values: Vec<(String, Vec<(isize, isize)>)> = vec![
            (
                "R3,D2,L4".to_string(),
                vec![(0, 0), (3, 0), (3, -2), (-1, -2)],
            )
        ];
        for (x, res) in values {
            assert_eq!(
                parse_path(&x),
                res
                    .into_iter()
                    .map(|(x, y)| Coordinate { x, y })
                    .collect::<Vec<Coordinate>>()
            );
        }
    }

    #[test]
    fn test_find_closest_intersection_by_manhattan_distance() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let (wire0, wire1) = parse_input(input);
        assert_eq!(
            manhattan_distance(&find_closest_intersection_by_manhattan_distance(&wire0, &wire1).unwrap(), &ZERO_COORDINATE),
            159,
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), Some(806));
    }

    #[test]
    fn test_find_closest_intersection_by_wire_length() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83".to_string();
        let (wire0, wire1) = parse_input(input);
        assert_eq!(find_closest_intersection_by_wire_length(&wire0, &wire1), Some((Coordinate { x: 158, y: -12 }, 610)));
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), Some(66076));
    }
}