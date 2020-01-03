use std::collections::{HashMap, HashSet};

use super::intcode::Computer;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Color {
    BLACK = 0,
    WHITE = 1,
}

impl From<i64> for Color {
    fn from(x: i64) -> Self {
        match x {
            0 => Color::BLACK,
            1 => Color::WHITE,
            _ => panic!("wtf lol not a color {}", x),
        }
    }
}

#[derive(Debug)]
enum Turn {
    LEFT = 0,
    RIGHT = 1,
}

impl From<i64> for Turn {
    fn from(x: i64) -> Self {
        match x {
            0 => Turn::LEFT,
            1 => Turn::RIGHT,
            _ => panic!("wtf lol not a turn {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    pub fn up() -> Self {
        Direction { dx: 0, dy: 1 }
    }
    pub fn right() -> Self {
        Direction { dx: 1, dy: 0 }
    }
    pub fn down() -> Self {
        Direction { dx: 0, dy: -1 }
    }
    pub fn left() -> Self {
        Direction { dx: -1, dy: 0 }
    }
    pub fn turn(&self, turn: &Turn) -> Self {
        match (self.dx, self.dy, turn) {
            (1, 0, Turn::LEFT) | (-1, 0, Turn::RIGHT) => Direction::up(),
            (1, 0, Turn::RIGHT) | (-1, 0, Turn::LEFT) => Direction::down(),
            (0, 1, Turn::RIGHT) | (0, -1, Turn::LEFT) => Direction::right(),
            (0, 1, Turn::LEFT) | (0, -1, Turn::RIGHT) => Direction::left(),
            _ => panic!("movement not understood! {:?}, {:?}", self, turn),
        }
    }
}

type Map = HashMap<(isize, isize), Color>;

struct Position {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Position {
    pub fn next(&self) -> Self {
        Position {
            x: self.x + self.direction.dx,
            y: self.y + self.direction.dy,
            direction: self.direction,
        }
    }
    pub fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

struct Robot {
    computer: Computer,
    position: Position,
    map: Map,
}

impl Robot {
    fn new() -> Self {
        Robot {
            computer: Computer::new(&Computer::load_data(11), &[]),
            map: HashMap::new(),
            position: Position {
                x: 0,
                y: 0,
                direction: Direction::up(),
            },
        }
    }
    fn input_current_color(&mut self, color: &Color) {
        self.computer.input_queue.push_back((*color) as i64);
    }
    fn paint(&mut self) -> &Map {
        self.last();
        &self.map
    }
}

impl Iterator for Robot {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        let current_color = *self
            .map
            .entry(self.position.as_tuple())
            .or_insert(Color::BLACK);
        self.input_current_color(&current_color);
        let color_to_paint = {
            if let Some(next_color) = self.computer.next() {
                Color::from(next_color)
            } else {
                return None;
            }
        };
        let turn = Turn::from(self.computer.next().unwrap());
        self.map.insert(self.position.as_tuple(), color_to_paint);
        self.position.direction = self.position.direction.turn(&turn);
        let last_position = self.position.as_tuple();
        self.position = self.position.next();
        Some(last_position)
    }
}

fn print_map(map: &Map) {
    let bounds = {
        let mut min_x = isize::max_value();
        let mut min_y = isize::max_value();
        let mut max_x = isize::min_value();
        let mut max_y = isize::min_value();
        for (x, y) in map.keys() {
            min_x = std::cmp::min(*x, min_x);
            min_y = std::cmp::min(*y, min_y);
            max_x = std::cmp::max(*x, max_x);
            max_y = std::cmp::max(*y, max_y);
        }
        (min_x, max_x, min_y, max_y)
    };

    for y in (bounds.2..bounds.3 + 1).rev() {
        for x in bounds.0..bounds.1 + 1 {
            let value = map.get(&(x, y)).unwrap_or(&Color::BLACK);
            match value {
                Color::BLACK => print!("  "),
                Color::WHITE => print!("██"),
            }
        }
        println!();
    }
}

pub fn solve_part_one() -> usize {
    let robot = Robot::new();
    robot.collect::<HashSet<(isize, isize)>>().len()
}

pub fn solve_part_two() {
    let mut robot = Robot::new();
    robot.map.insert((0, 0), Color::WHITE);
    let map = robot.paint();
    println!("day 11 part two: ");
    print_map(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 1686);
    }

    #[test]
    fn test_solve_part_two() {
        solve_part_two();
    }
}
