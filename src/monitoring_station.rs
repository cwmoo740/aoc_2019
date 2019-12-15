use std::collections::{HashSet, HashMap};

use num::integer::gcd;
use itertools::Itertools;

type Coordinate = (usize, usize);
type Slope = (isize, isize);
type AsteroidMap = HashSet<Coordinate>;

fn destroyed_asteroids(asteroids: &AsteroidMap, laser_coordinate: &Coordinate) -> Vec<Coordinate> {
    let mut categories: HashMap<Slope, Vec<Coordinate>> = asteroids
        .iter()
        .filter(|&coord| *coord != *laser_coordinate)
        .sorted_by(|a, b| {
            let a_dist = distance_between(laser_coordinate, &a);
            let b_dist = distance_between(laser_coordinate, &b);
            b_dist.partial_cmp(&a_dist).unwrap()
        })
        .map(|coord| (coord, slope_between(laser_coordinate, &coord)))
        .fold(
            HashMap::new(),
            |mut acc, (coord, slope)| {
                acc.entry(slope).or_insert(Vec::new()).push(*coord);
                acc
            },
        );

    let keys = categories
        .keys()
        .sorted_by(|(dx0, dy0), (dx1, dy1)| {
            let a = (*dx0 as f32).atan2(*dy0 as f32);
            let b = (*dx1 as f32).atan2(*dy1 as f32);
            b.partial_cmp(&a).unwrap()
        })
        .cloned()
        .collect::<Vec<Slope>>();

    let mut result: Vec<Coordinate> = Vec::new();
    while result.len() < asteroids.len() - 1 {
        for key in keys.clone() {
            let entry: &mut Vec<Coordinate> = categories.get_mut(&key).unwrap();
            if !entry.is_empty() {
                result.push(entry.pop().unwrap());
            }
        }
    }
    result
}

fn slope_between(a: &Coordinate, b: &Coordinate) -> Slope {
    let dx = b.0 as isize - a.0 as isize;
    let dy = b.1 as isize - a.1 as isize;
    let div = gcd(dx, dy);
    (dx / div, dy / div)
}

fn distance_between(a: &Coordinate, b: &Coordinate) -> f32 {
    let dx = ((b.0 as isize - a.0 as isize) as f32).powi(2);
    let dy = ((b.1 as isize - a.1 as isize) as f32).powi(2);
    dx + dy
}

fn visible_asteroids(asteroids: &AsteroidMap, coordinate: &Coordinate) -> usize {
    asteroids
        .iter()
        .filter(|&&x| x != *coordinate)
        .map(|x| slope_between(coordinate, x))
        .collect::<HashSet<Slope>>()
        .len()
}

fn best_asteroid(asteroids: &AsteroidMap) -> Coordinate {
    *asteroids
        .iter()
        .max_by(|&a, &b| {
            let a_score = visible_asteroids(asteroids, a);
            let b_score = visible_asteroids(asteroids, b);
            a_score.cmp(&b_score)
        })
        .unwrap()
}

fn parse_map(input: &str) -> AsteroidMap {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .trim()
                .chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

pub fn solve_part_one() -> usize {
    let map = parse_map(&super::get_input::main(10));
    let best_station = best_asteroid(&map);
    visible_asteroids(&map, &best_station)
}

pub fn solve_part_two() -> usize {
    let map = parse_map(&super::get_input::main(10));
    let best_station = best_asteroid(&map);
    let destroyed = destroyed_asteroids(&map, &best_station);
    let (x, y) = destroyed[199];
    x * 100 + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let input = ".#..#
                           .....
                           #####
                           ....#
                           ...##";
        assert_eq!(
            parse_map(input),
            [(1, 0), (4, 0), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (4, 3), (3, 4), (4, 4)]
                .iter()
                .cloned()
                .collect::<AsteroidMap>()
        );
    }

    #[test]
    fn test_visible_asteroid() {
        let input = ".#..#
                           .....
                           #####
                           ....#
                           ...##";
        let map = parse_map(input);
        let values: Vec<(Coordinate, usize)> = vec![
            ((1, 0), 7),
            ((4, 0), 7),
            ((0, 2), 6),
            ((4, 2), 5),
            ((3, 4), 8),
        ];
        for (coord, expected) in values {
            assert_eq!(visible_asteroids(&map, &coord), expected);
        }
    }

    #[test]
    fn test_best_asteroid() {
        let values: Vec<(&str, (usize, usize))> = vec![
            (
                ".#..#
                 .....
                 #####
                 ....#
                 ...##",
                (3, 4),
            ),
            (
                "#.#...#.#.
                 .###....#.
                 .#....#...
                 ##.#.#.#.#
                 ....#.#.#.
                 .##..###.#
                 ..#...##..
                 ..##....##
                 ......#...
                 .####.###.",
                (1, 2),
            )
        ];
        for (input, expected) in values {
            assert_eq!(best_asteroid(&parse_map(input)), expected);
        }
    }

    #[test]
    fn test_laser() {
        let input: &str = ".#....#####...#..
                           ##...##.#####..##
                           ##...#...#.#####.
                           ..#.....X...###..
                           ..#.#.....#....##";
        let asteroids = parse_map(input);
        assert_eq!(
            destroyed_asteroids(&asteroids, &(8, 3)).into_iter().take(9).collect::<Vec<Coordinate>>(),
            vec![(8, 1), (9, 0), (9, 1), (10, 0), (9, 2), (11, 1), (12, 1), (11, 2), (15, 1)]
        );
    }

    #[test]
    fn test_large_map() {
        let input: &str = ".#..##.###...#######
                           ##.############..##.
                           .#.######.########.#
                           .###.#######.####.#.
                           #####.##.#.##.###.##
                           ..#####..#.#########
                           ####################
                           #.####....###.#.#.##
                           ##.#################
                           #####.##.###..####..
                           ..######..##.#######
                           ####.##.####...##..#
                           .#####..#.######.###
                           ##...#.##########...
                           #.##########.#######
                           .####.#.###.###.#.##
                           ....##.##.###..#####
                           .#.#.###########.###
                           #.#.#.#####.####.###
                           ###.##.####.##.#..##";
        let map = parse_map(input);
        let best_coordinate = best_asteroid(&map);
        assert_eq!(best_coordinate, (11, 13));

        let destroyed = destroyed_asteroids(&map, &best_coordinate);
        let values: Vec<(usize, Coordinate)> = vec![
            (0, (11, 12)),
            (1, (12, 1)),
            (2, (12, 2)),
            (9, (12, 8)),
            (19, (16, 0)),
            (49, (16, 9)),
            (99, (10, 16)),
            (198, (9, 6)),
            (199, (8, 2)),
            (200, (10, 9)),
            (298, (11, 1)),
        ];
        for (i, coord) in values {
            assert_eq!(destroyed[i], coord);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 340);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 2628);
    }
}
