use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

static MAX_LEVELS: usize = 30;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vertex {
    x: usize,
    y: usize,
}

struct Game {
    map: HashMap<(Vertex, usize), HashSet<(Vertex, usize)>>,
    start: (Vertex, usize),
    end: (Vertex, usize),
}

impl Game {
    fn parse_input(
        input: &str,
    ) -> (
        HashMap<Vertex, HashSet<Vertex>>,
        HashMap<String, HashSet<Vertex>>,
        Vertex,
        Vertex,
    ) {
        let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

        let get_value = |x: usize, y: usize| -> char {
            *grid.get(y).unwrap_or(&Vec::new()).get(x).unwrap_or(&'#')
        };
        let mut map = HashMap::new();
        let mut portals: HashMap<String, HashSet<Vertex>> = HashMap::new();

        for y in 1..grid.len() - 1 {
            for x in 1..grid[y].len() - 1 {
                match grid[y][x] {
                    '.' => {
                        let possible_neighbors =
                            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                                .into_iter()
                                .filter(|&(a, b)| get_value(a, b) == '.');

                        for (a, b) in possible_neighbors {
                            map.entry(Vertex { x, y })
                                .or_insert(HashSet::new())
                                .insert(Vertex { x: a, y: b });
                            map.entry(Vertex { x: a, y: b })
                                .or_insert(HashSet::new())
                                .insert(Vertex { x, y });
                        }
                    }
                    ch if ch.is_ascii_uppercase() => {
                        //     top, right, bottom, left
                        match (
                            get_value(x, y - 1),
                            get_value(x + 1, y),
                            get_value(x, y + 1),
                            get_value(x - 1, y),
                        ) {
                            ('.', _, z, _) if z.is_ascii_uppercase() => {
                                portals
                                    .entry(format!("{}{}", ch, z))
                                    .or_insert(HashSet::new())
                                    .insert(Vertex { x, y: y - 1 });
                            }
                            (z, _, '.', _) if z.is_ascii_uppercase() => {
                                portals
                                    .entry(format!("{}{}", z, ch))
                                    .or_insert(HashSet::new())
                                    .insert(Vertex { x, y: y + 1 });
                            }
                            (_, z, _, '.') if z.is_ascii_uppercase() => {
                                portals
                                    .entry(format!("{}{}", ch, z))
                                    .or_insert(HashSet::new())
                                    .insert(Vertex { x: x - 1, y });
                            }
                            (_, '.', _, z) if z.is_ascii_uppercase() => {
                                portals
                                    .entry(format!("{}{}", z, ch))
                                    .or_insert(HashSet::new())
                                    .insert(Vertex { x: x + 1, y });
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
        let start = portals
            .remove("AA")
            .expect("there should be a start vertex in the portals map")
            .into_iter()
            .next()
            .expect("there should be a start vertex");

        let end = portals
            .remove("ZZ")
            .expect("there should be an end vertex in the portals map")
            .into_iter()
            .next()
            .expect("there should be an end vertex");
        return (map, portals, start, end);
    }
    fn new(input: &str) -> Game {
        let (mut map, portals, start, end) = Game::parse_input(input);

        for (_, vertices) in portals.into_iter() {
            for (&a, &b) in vertices
                .iter()
                .cartesian_product(vertices.iter())
                .filter(|(a, b)| a != b)
            {
                map.entry(a).or_insert(HashSet::new()).insert(b);
            }
        }
        Game {
            map: map
                .into_iter()
                .map(|(key, vertices)| ((key, 0), vertices.into_iter().map(|v| (v, 0)).collect()))
                .collect(),
            start: (start, 0),
            end: (end, 0),
        }
    }
    fn new_recursive(input: &str) -> Game {
        let (passages, portals, start, end) = Game::parse_input(input);

        let (x_min, x_max) = passages
            .keys()
            .minmax_by_key(|z| z.x)
            .into_option()
            .map(|(a, b)| (a.x, b.x))
            .expect("there should be a minimum and maximum x bound");
        let (y_min, y_max) = passages
            .keys()
            .minmax_by_key(|z| z.y)
            .into_option()
            .map(|(a, b)| (a.y, b.y))
            .expect("there should be a minimum and maximum y bound");

        let mut map: HashMap<(Vertex, usize), HashSet<(Vertex, usize)>> = HashMap::new();
        for (&parent, children) in passages.iter() {
            for &child in children {
                for level in 0..MAX_LEVELS {
                    map.entry((parent, level))
                        .or_insert(HashSet::new())
                        .insert((child, level));
                }
            }
        }

        for (_, vertices) in portals.into_iter() {
            for (&a, &b) in vertices
                .iter()
                .cartesian_product(vertices.iter())
                .filter(|(a, b)| a != b)
            {
                let (inner, outer) = {
                    if a.x == x_min || a.x == x_max || a.y == y_min || a.y == y_max {
                        (b, a)
                    } else {
                        (a, b)
                    }
                };
                for level in 0..MAX_LEVELS - 1 {
                    map.entry((inner, level))
                        .or_insert(HashSet::new())
                        .insert((outer, level + 1));
                    map.entry((outer, level + 1))
                        .or_insert(HashSet::new())
                        .insert((inner, level));
                }
            }
        }

        Game {
            map,
            start: (start, 0),
            end: (end, 0),
        }
    }
    fn solve(&self) -> (Vec<(Vertex, usize)>, usize) {
        dijkstra(
            &self.start,
            |v| {
                self.map
                    .get(v)
                    .map(|z| z.iter().map(|&neighbor| (neighbor, 1)).collect_vec())
                    .unwrap_or(Vec::new())
            },
            |&v| v == self.end,
        )
        .expect("the maze should be solvable")
    }
}

pub fn solve_part_one() -> usize {
    let game = Game::new(&super::get_input::main(20));
    game.solve().1
}

pub fn solve_part_two() -> usize {
    let game = Game::new_recursive(&super::get_input::main(20));
    game.solve().1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_puzzle() -> String {
        concat!(
            "         A         \n",
            "         A         \n",
            "  #######.#########\n",
            "  #######.........#\n",
            "  #######.#######.#\n",
            "  #######.#######.#\n",
            "  #######.#######.#\n",
            "  #####  B    ###.#\n",
            "BC...##  C    ###.#\n",
            "  ##.##       ###.#\n",
            "  ##...DE  F  ###.#\n",
            "  #####    G  ###.#\n",
            "  #########.#####.#\n",
            "DE..#######...###.#\n",
            "  #.#########.###.#\n",
            "FG..#########.....#\n",
            "  ###########.#####\n",
            "             Z     \n",
            "             Z     \n",
        )
        .to_string()
    }

    #[test]
    fn test_parse_input() {
        let game = Game::new(&simple_puzzle());
        assert_eq!(game.start, (Vertex { x: 9, y: 2 }, 0));
        assert_eq!(game.end, (Vertex { x: 13, y: 16 }, 0));
        assert_eq!(
            game.map.get(&(Vertex { x: 9, y: 2 }, 0)),
            Some(&vec![(Vertex { x: 9, y: 3 }, 0)].into_iter().collect())
        );
        assert_eq!(
            game.map.get(&(Vertex { x: 9, y: 6 }, 0)),
            Some(
                &vec![(9, 5), (2, 8)]
                    .into_iter()
                    .map(|(x, y)| (Vertex { x, y }, 0))
                    .collect()
            )
        )
    }

    #[test]
    fn test_solve_game() {
        let input = simple_puzzle();
        let game = Game::new(&input);
        assert_eq!(game.solve().1, 23);

        let game_recursive = Game::new_recursive(&input);
        assert_eq!(game_recursive.solve().1, 26)
    }

    #[test]
    fn test_complicated_recursive_solve() {
        let input = concat!(
            "             Z L X W       C                 \n",
            "             Z P Q B       K                 \n",
            "  ###########.#.#.#.#######.###############  \n",
            "  #...#.......#.#.......#.#.......#.#.#...#  \n",
            "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
            "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
            "  #.###.#######.###.###.#.###.###.#.#######  \n",
            "  #...#.......#.#...#...#.............#...#  \n",
            "  #.#########.#######.#.#######.#######.###  \n",
            "  #...#.#    F       R I       Z    #.#.#.#  \n",
            "  #.###.#    D       E C       H    #.#.#.#  \n",
            "  #.#...#                           #...#.#  \n",
            "  #.###.#                           #.###.#  \n",
            "  #.#....OA                       WB..#.#..ZH\n",
            "  #.###.#                           #.#.#.#  \n",
            "CJ......#                           #.....#  \n",
            "  #######                           #######  \n",
            "  #.#....CK                         #......IC\n",
            "  #.###.#                           #.###.#  \n",
            "  #.....#                           #...#.#  \n",
            "  ###.###                           #.#.#.#  \n",
            "XF....#.#                         RF..#.#.#  \n",
            "  #####.#                           #######  \n",
            "  #......CJ                       NM..#...#  \n",
            "  ###.#.#                           #.###.#  \n",
            "RE....#.#                           #......RF\n",
            "  ###.###        X   X       L      #.#.#.#  \n",
            "  #.....#        F   Q       P      #.#.#.#  \n",
            "  ###.###########.###.#######.#########.###  \n",
            "  #.....#...#.....#.......#...#.....#.#...#  \n",
            "  #####.#.###.#######.#######.###.###.#.#.#  \n",
            "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
            "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
            "  #.......#.....#.#...#...............#...#  \n",
            "  #############.#.#.###.###################  \n",
            "               A O F   N                     \n",
            "               A A D   M                     "
        );
        let game = Game::new_recursive(input);
        assert_eq!(game.solve().1, 396);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 526);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 6292);
    }
}
