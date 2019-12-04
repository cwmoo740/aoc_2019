extern crate regex;

use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::Add;
use std::iter::Map;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    R,
    D,
    L,
    U,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Add<&Move> for Coordinate {
    type Output = Vec<Coordinate>;

    fn add(&self, m: &Move) -> Vec<Coordinate> {
        match m.direction {
            Direction::R => {
                (1..(m.magnitude + 1))
                    .map(|dx| Coordinate { x: self.x + dx, y: self.y })
                    .collect()
            }
            Direction::D => {
                (1..(m.magnitude + 1))
                    .map(|dy| Coordinate { x: self.x, y: self.y - dy })
                    .collect()
            }
            Direction::L => {
                (1..(m.magnitude + 1))
                    .map(|dx| Coordinate { x: self.x - dx, y: self.y })
                    .collect()
            }
            Direction::U => {
                (1..(m.magnitude + 1))
                    .map(|dy| Coordinate { x: self.x, y: self.y + dy })
                    .collect()
            }
        }
    }
}

impl Coordinate {
    fn manhattan_distance(&self) -> usize {
        (self.x.abs() as usize) + (self.y.abs() as usize)
    }

}

fn parse_path(input: &String) -> Vec<Coordinate> {
    input
        .split(",")
        .map(Move::from_str)
        .collect::<Result<Vec<Move>, _>>()
        .unwrap()
        .into_iter()
        .fold(
            vec![Coordinate { x: 0, y: 0 }],
            |mut acc: Vec<Coordinate>, nex: Move| {
                let items = *acc.last().unwrap() + &nex;
                acc.extend(items);
                acc
            },
        )
}

fn solve_part_one() -> Coordinate {

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
                vec![(0, 0), (1, 0), (2, 0), (3, 0), (3, -1), (3, -2), (2, -2), (1, -2), (0, -2), (-1, -2)],
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
}