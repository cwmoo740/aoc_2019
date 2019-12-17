use std::cmp::Ordering;
use std::ops::Add;

use regex::Regex;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Velocity {
    dx: isize,
    dy: isize,
    dz: isize,
}

impl Add<Acceleration> for Velocity {
    type Output = Velocity;
    fn add(self, rhs: Acceleration) -> Self::Output {
        Velocity {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz,
        }
    }
}

impl Velocity {
    fn energy(&self) -> usize {
        (self.dx.abs() + self.dy.abs() + self.dz.abs()) as usize
    }
}

type Acceleration = Velocity;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

// new_position = position + velocity
impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, rhs: Velocity) -> Self::Output {
        Position {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
            z: self.z + rhs.dz,
        }
    }
}

impl From<&str> for Position {
    fn from(x: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        }
        match RE.captures(x.trim()) {
            Some(cap) => Position {
                x: isize::from_str_radix(&cap[1], 10).unwrap(),
                y: isize::from_str_radix(&cap[2], 10).unwrap(),
                z: isize::from_str_radix(&cap[3], 10).unwrap(),
            },
            None => panic!("can't parse this line: {}", x),
        }
    }
}

impl Position {
    fn energy(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    #[cfg(test)]
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon {
            velocity: Velocity::default(),
            position: Position {
                x,
                y,
                z,
            },
        }
    }
    #[cfg(test)]
    fn with_velocity(&mut self, dx: isize, dy: isize, dz: isize) -> Self {
        self.velocity = Velocity { dx, dy, dz };
        *self
    }
    fn energy(&self) -> usize {
        self.velocity.energy() * self.position.energy()
    }
}

fn compare_axis(a: isize, b: isize) -> (isize, isize) {
    match a.cmp(&b) {
        Ordering::Greater => (-1, 1),
        Ordering::Less => (1, -1),
        Ordering::Equal => (0, 0)
    }
}

fn gravity(a: &Position, b: &Position) -> (Acceleration, Acceleration) {
    let (dxa, dxb) = compare_axis(a.x, b.x);
    let (dya, dyb) = compare_axis(a.y, b.y);
    let (dza, dzb) = compare_axis(a.z, b.z);
    (
        Acceleration {
            dx: dxa,
            dy: dya,
            dz: dza,
        },
        Acceleration {
            dx: dxb,
            dy: dyb,
            dz: dzb,
        },
    )
}

struct SolarSystem {
    moons: Vec<Moon>,
}

impl Iterator for SolarSystem {
    type Item = Vec<Moon>;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.moons.len() {
            for j in i + 1..self.moons.len() {
                let (di, dj) = gravity(&self.moons[i].position, &self.moons[j].position);
                self.moons[i].velocity = di + self.moons[i].velocity;
                self.moons[j].velocity = dj + self.moons[j].velocity;
            }
        }
        for i in 0..self.moons.len() {
            self.moons[i].position = self.moons[i].position + self.moons[i].velocity;
        }
        Some(self.moons.to_vec())
    }
}

impl SolarSystem {
    fn energy(&self) -> usize {
        self.moons.iter().map(|m| m.energy()).sum()
    }
    fn equal_per_dim(&self, other: &SolarSystem) -> (bool, bool, bool) {
        if self.moons.len() != other.moons.len() {
            return (false, false, false);
        }
        self.moons
            .iter()
            .zip(other.moons.iter())
            .map(|(a, b)| (
                a.position.x == b.position.x && a.velocity.dx == b.velocity.dx,
                a.position.y == b.position.y && a.velocity.dy == b.velocity.dy,
                a.position.z == b.position.z && a.velocity.dz == b.velocity.dz,
            ))
            .fold(
                (true, true, true),
                |(x_r, y_r, z_r), (x, y, z)| (
                    x_r && x,
                    y_r && y,
                    z_r && z
                ),
            )
    }
}

impl From<String> for SolarSystem {
    fn from(x: String) -> Self {
        let moons = x
            .trim()
            .lines()
            .map(|line| Moon { position: Position::from(line), velocity: Velocity::default() })
            .collect::<Vec<Moon>>();
        SolarSystem { moons }
    }
}

fn identify_cycles(mut system: SolarSystem) -> (usize, usize, usize) {
    let original = SolarSystem {
        moons: system.moons.to_vec(),
    };
    let mut cycles = (None, None, None);
    let mut i = 0;
    while cycles.0.is_none() || cycles.1.is_none() || cycles.2.is_none() {
        i += 1;
        system.next();
        let (equal_x, equal_y, equal_z) = system.equal_per_dim(&original);
        if equal_x && cycles.0.is_none() {
            cycles.0 = Some(i);
        }
        if equal_y && cycles.1.is_none() {
            cycles.1 = Some(i);
        }
        if equal_z && cycles.2.is_none() {
            cycles.2 = Some(i);
        }
    }
    (cycles.0.unwrap(), cycles.1.unwrap(), cycles.2.unwrap())
}

pub fn solve_part_one() -> usize {
    let mut solar_system = SolarSystem::from(super::get_input::main(12));
    solar_system.nth(999);
    solar_system.energy()
}

pub fn solve_part_two() -> usize {
    let solar_system = SolarSystem::from(super::get_input::main(12));
    let (cx, cy, cz) = identify_cycles(solar_system);
    num::integer::lcm(cx, num::integer::lcm(cy, cz))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solar_system() {
        let mut system = SolarSystem {
            moons: vec![
                Moon::new(-1, 0, 2),
                Moon::new(2, -10, -7),
                Moon::new(4, -8, 8),
                Moon::new(3, 5, -1),
            ],
        };
        assert_eq!(
            system.next().unwrap(),
            vec![
                Moon::new(2, -1, 1).with_velocity(3, -1, -1),
                Moon::new(3, -7, -4).with_velocity(1, 3, 3),
                Moon::new(1, -7, 5).with_velocity(-3, 1, -3),
                Moon::new(2, 2, 0).with_velocity(-1, -3, 1),
            ]
        );
        assert_eq!(
            system.nth(8).unwrap(),
            vec![
                Moon::new(2, 1, -3).with_velocity(-3, -2, 1),
                Moon::new(1, -8, 0).with_velocity(-1, 1, 3),
                Moon::new(3, -6, 1).with_velocity(3, 2, -3),
                Moon::new(2, 0, 4).with_velocity(1, -1, -1),
            ]
        );
        assert_eq!(system.energy(), 179);
    }

    #[test]
    fn test_identify_cycles() {
        let system = SolarSystem {
            moons: vec![
                Moon::new(-1, 0, 2),
                Moon::new(2, -10, -7),
                Moon::new(4, -8, 8),
                Moon::new(3, 5, -1),
            ],
        };
        assert_eq!(identify_cycles(system), (18, 28, 44));
        assert_eq!(num::integer::lcm(18, num::integer::lcm(28, 44)), 2772);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 14780);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 279751820342592);
    }
}