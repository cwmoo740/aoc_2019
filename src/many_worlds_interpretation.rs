use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

type Grid = HashMap<Vertex, Tile>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vertex {
    x: isize,
    y: isize,
}

impl Vertex {
    fn new(x: isize, y: isize) -> Vertex {
        Vertex { x, y }
    }
    fn neighbors(&self) -> Vec<Vertex> {
        let (x, y) = (self.x, self.y);
        vec![
            Vertex::new(x + 1, y),
            Vertex::new(x - 1, y),
            Vertex::new(x, y + 1),
            Vertex::new(x, y - 1),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct SingleRobotState(Vertex, String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct MultiRobotState(Vec<Vertex>, Option<usize>, String);

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Passage,
    Robot,
    Wall,
    Key(char),
    Door(char),
}

impl From<char> for Tile {
    fn from(x: char) -> Tile {
        match x {
            '@' => Tile::Robot,
            '#' => Tile::Wall,
            '.' => Tile::Passage,
            c if 65 <= (c as u8) && (c as u8) <= 90 => Tile::Door(c),
            c if 97 <= (c as u8) && (c as u8) <= 122 => Tile::Key(c),
            c => panic!("tile not recognized: {}", c),
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Robot => "@".to_string(),
            Tile::Wall => "#".to_string(),
            Tile::Passage => ".".to_string(),
            &Tile::Door(c) | &Tile::Key(c) => c.to_string(),
        }
    }
}

fn parse_input(input: String, modify_grid: bool) -> (Grid, Vec<Vertex>) {
    let values = input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let mut grid: Grid = HashMap::new();
    let mut robots: Vec<Vertex> = Vec::new();

    for (y, row) in values.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            match tile {
                Tile::Robot => {
                    robots.push(Vertex::new(x as isize, y as isize));
                    grid.insert(Vertex::new(x as isize, y as isize), Tile::Passage);
                }
                _ => {
                    grid.insert(Vertex::new(x as isize, y as isize), tile);
                }
            };
        }
    }

    if modify_grid {
        if let Some(Vertex { x, y }) = robots.pop() {
            grid.insert(Vertex::new(x, y), Tile::Wall);
            grid.insert(Vertex::new(x - 1, y), Tile::Wall);
            grid.insert(Vertex::new(x + 1, y), Tile::Wall);
            grid.insert(Vertex::new(x, y - 1), Tile::Wall);
            grid.insert(Vertex::new(x, y + 1), Tile::Wall);
            robots.push(Vertex::new(x - 1, y - 1));
            robots.push(Vertex::new(x - 1, y + 1));
            robots.push(Vertex::new(x + 1, y - 1));
            robots.push(Vertex::new(x + 1, y + 1));
        }
        for &robot in robots.iter() {
            grid.insert(robot, Tile::Passage);
        }
    }

    (grid, robots)
}

fn grid_to_string(grid: &Grid, robots: &Vec<Vertex>) -> String {
    let mut result: Vec<String> = Vec::new();
    let x_max = grid.keys().max_by_key(|&v| v.x).unwrap().x;
    let y_max = grid.keys().max_by_key(|&v| v.y).unwrap().y;
    for y in 0..y_max + 1 {
        let mut inner: Vec<String> = Vec::new();
        for x in 0..x_max + 1 {
            inner.push(
                grid.get(&Vertex { x, y })
                    .map(|z| z.to_string())
                    .unwrap_or(" ".to_string()),
            );
        }
        result.push(inner.join(""));
    }
    result.join("\n")
}

fn get_all_keys(grid: &Grid) -> String {
    grid.values()
        .filter_map(|z| match z {
            &Tile::Key(c) => Some(c.to_string()),
            _ => None,
        })
        .sorted()
        .join("")
}

fn add_key(keys: &str, c: char) -> String {
    let mut cloned = keys.to_string();
    if keys.contains(&c.to_string()) {
        cloned
    } else {
        let x = keys.chars().position(|z| c < z).unwrap_or(keys.len());
        cloned.insert(x, c);
        cloned
    }
}

fn update_robot_positions(positions: &Vec<Vertex>, new_vertex: Vertex, index: usize) -> Vec<Vertex> {
    let mut new_positions = positions.clone();
    new_positions[index] = new_vertex;
    new_positions
}

fn get_successors(grid: &Grid, positions: &Vec<Vertex>, active_robot: usize, keys: &str) -> Vec<MultiRobotState> {
    positions[active_robot].neighbors().into_iter()
        .filter_map(|neighbor| match grid.get(&neighbor) {
            Some(&Tile::Key(key)) => Some(MultiRobotState(update_robot_positions(positions, neighbor, active_robot), None, add_key(keys, key))),
            Some(&Tile::Door(code)) if keys.contains(&code.to_ascii_lowercase().to_string()) => Some(MultiRobotState(update_robot_positions(positions, neighbor, active_robot), Some(active_robot), keys.to_string())),
            Some(Tile::Passage) => Some(MultiRobotState(update_robot_positions(positions, neighbor, active_robot), Some(active_robot), keys.to_string())),
            _ => None,
        })
        .collect()
}

fn shortest_walk(grid: &Grid, robots: &Vec<Vertex>) -> usize {
    let all_keys = get_all_keys(grid);
    let start_node = MultiRobotState(robots.clone(), None, String::new());
    let (_, cost) = dijkstra(
        &start_node,
        |MultiRobotState(vertices, active_robot, keys)| {
            {
                if let &Some(robot_index) = active_robot {
                    get_successors(grid, vertices, robot_index, keys)
                } else {
                    (0..vertices.len())
                        .flat_map(|i| get_successors(grid, vertices, i, keys))
                        .collect()
                }
            }
                .into_iter()
                .map(|z| (z, 1))
                .collect::<Vec<(MultiRobotState, usize)>>()
        },
        |MultiRobotState(_, _, keys)| *keys == all_keys,
    ).unwrap();
    cost
}


pub fn solve_part_one() -> usize {
    let input = super::get_input::main(18);
    let (grid, start_vertices) = parse_input(input, false);
    shortest_walk(&grid, &start_vertices)
}

pub fn solve_part_two() -> usize {
    let input = super::get_input::main(18);
    let (grid, start_vertices) = parse_input(input, true);
    shortest_walk(&grid, &start_vertices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        let input = concat!("#########\n", "#b.A.@.a#\n", "#########", ).to_string();
        let (grid, robots) = parse_input(input, false);
        assert_eq!(robots, vec![Vertex::new(5, 1)]);
        assert_eq!(
            grid,
            vec![
                ((1, 1), Tile::Key('b')),
                ((2, 1), Tile::Passage),
                ((3, 1), Tile::Door('A')),
                ((4, 1), Tile::Passage),
                ((5, 1), Tile::Passage),
                ((6, 1), Tile::Passage),
                ((7, 1), Tile::Key('a')),
            ]
                .into_iter()
                .map(|((x, y), tile)| (Vertex::new(x, y), tile))
                .collect::<HashMap<Vertex, Tile>>()
        );
    }

    #[test]
    fn test_shortest_walk() {
        let (grid, start_vertices) = parse_input(
            concat!(
            "########################\n",
            "#...............b.C.D.f#\n",
            "#.######################\n",
            "#.....@.a.B.c.d.A.e.F.g#\n",
            "########################",
            )
                .to_string(),
            false,
        );
        assert_eq!(shortest_walk(&grid, &start_vertices), 132);
    }

    #[test]
    fn test_shortest_walk_with_multiple_robots() {
        let (grid, start_vertices) = parse_input(
            concat!(
            "###############\n",
            "#d.ABC.#.....a#\n",
            "######...######\n",
            "######.@.######\n",
            "######...######\n",
            "#b.....#.....c#\n",
            "###############\n",
            )
                .to_string(),
            true,
        );
        assert_eq!(shortest_walk(&grid, &start_vertices), 24);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 5402);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 2138);
    }
}
