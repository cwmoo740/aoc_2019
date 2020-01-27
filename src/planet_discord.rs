use std::collections::{HashMap, HashSet};

struct NonRecursivePuzzle {
    bounds: (usize, usize),
    bugs: Vec<bool>,
    seen_biodiversity: HashSet<u64>,
}

impl From<&String> for NonRecursivePuzzle {
    fn from(x: &String) -> Self {
        let bounds: (usize, usize) = (
            x.trim().lines().count(),
            x.trim()
                .lines()
                .map(|line| line.trim().chars().count())
                .max()
                .unwrap(),
        );

        let bugs: Vec<bool> = x
            .trim()
            .lines()
            .flat_map(|line| {
                line.trim().chars().map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unrecognized character in input"),
                })
            })
            .collect();

        NonRecursivePuzzle {
            bounds,
            bugs,
            seen_biodiversity: HashSet::new(),
        }
    }
}

impl NonRecursivePuzzle {
    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i / self.bounds.1, i % self.bounds.1)
    }

    #[cfg(test)]
    fn bug_positions(&self) -> Vec<(usize, usize)> {
        self.bugs
            .iter()
            .enumerate()
            .filter(|(_, &is_bug)| is_bug)
            .map(|(i, _)| self.index_to_xy(i))
            .collect()
    }

    fn count_adjacent_bugs(&self, i: usize) -> usize {
        let mut result: usize = 0;
        let (x, y) = self.index_to_xy(i);
        if x > 0 && self.bugs[i - self.bounds.1] {
            result += 1;
        }
        if x < self.bounds.0 - 1 && self.bugs[i + self.bounds.1] {
            result += 1;
        }
        if y > 0 && self.bugs[i - 1] {
            result += 1;
        }
        if y < self.bounds.1 - 1 && self.bugs[i + 1] {
            result += 1;
        }
        result
    }

    fn update(&mut self) {
        let mut bugs: HashSet<usize> = HashSet::new();
        for (i, &is_bug) in self.bugs.iter().enumerate() {
            let adjacent_bugs = self.count_adjacent_bugs(i);
            if adjacent_bugs == 1 {
                bugs.insert(i);
            } else if !is_bug && adjacent_bugs == 2 {
                bugs.insert(i);
            }
        }
        for i in 0..self.bugs.len() {
            self.bugs[i] = bugs.contains(&i);
        }
    }
    fn biodiversity(&self) -> u64 {
        self.bugs
            .iter()
            .enumerate()
            .filter(|(_, &is_bug)| is_bug)
            .map(|(i, _)| 2u64.pow(i as u32))
            .sum()
    }
    fn run_until_duplicate(&mut self) -> u64 {
        loop {
            let current_biodiversity = self.biodiversity();
            if self.seen_biodiversity.contains(&current_biodiversity) {
                return current_biodiversity;
            }
            self.seen_biodiversity.insert(current_biodiversity);
            self.update();
        }
    }
}

struct RecursivePuzzle {
    bugs: HashMap<isize, HashSet<(usize, usize)>>,
    grid_size: (usize, usize),
    levels: (isize, isize),
}

impl From<&String> for RecursivePuzzle {
    fn from(x: &String) -> Self {
        let grid_size: (usize, usize) = (
            x.trim().lines().count(),
            x.trim()
                .lines()
                .map(|line| line.trim().chars().count())
                .max()
                .unwrap(),
        );

        let level_zero_bugs = x
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(move |(y, ch)| match ch {
                        '#' => Some((x, y)),
                        '.' => None,
                        _ => panic!("unrecognized character in input"),
                    })
            })
            .collect();

        let mut bugs = HashMap::new();
        bugs.insert(0, level_zero_bugs);

        RecursivePuzzle {
            grid_size,
            bugs,
            levels: (0, 0),
        }
    }
}

impl RecursivePuzzle {
    fn get_neighbors(&self, level: isize, (x, y): (usize, usize)) -> Vec<(isize, usize, usize)> {
        let mut neighbors: Vec<(isize, usize, usize)> = Vec::new();

        if x == self.grid_size.0 / 2 && y == self.grid_size.1 / 2 {
            return neighbors;
        }

        if x == 0 {
            neighbors.push((level - 1, self.grid_size.0 / 2 - 1, self.grid_size.1 / 2));
        } else if x == self.grid_size.0 - 1 {
            neighbors.push((level - 1, self.grid_size.0 / 2 + 1, self.grid_size.1 / 2));
        }

        if x != 0 && !(x == self.grid_size.0 / 2 + 1 && y == self.grid_size.1 / 2) {
            neighbors.push((level, x - 1, y));
        }

        if !(x == self.grid_size.0 / 2 - 1 && y == self.grid_size.1 / 2)
            && x != self.grid_size.0 - 1
        {
            neighbors.push((level, x + 1, y));
        }

        if y == 0 {
            neighbors.push((level - 1, self.grid_size.0 / 2, self.grid_size.1 / 2 - 1));
        } else if y == self.grid_size.1 - 1 {
            neighbors.push((level - 1, self.grid_size.0 / 2, self.grid_size.1 / 2 + 1));
        }

        if y != 0 && !(y == self.grid_size.1 / 2 + 1 && x == self.grid_size.1 / 2) {
            neighbors.push((level, x, y - 1));
        }

        if !(y == self.grid_size.1 / 2 - 1 && x == self.grid_size.0 / 2)
            && y != self.grid_size.1 - 1
        {
            neighbors.push((level, x, y + 1));
        }

        if x == self.grid_size.0 / 2 - 1 && y == self.grid_size.1 / 2 {
            neighbors.extend((0..self.grid_size.1).map(|new_y| (level + 1, 0, new_y)));
        } else if x == self.grid_size.0 / 2 && y == self.grid_size.1 / 2 + 1 {
            neighbors.extend(
                (0..self.grid_size.0).map(|new_x| (level + 1, new_x, self.grid_size.1 - 1)),
            );
        } else if x == self.grid_size.0 / 2 + 1 && y == self.grid_size.1 / 2 {
            neighbors.extend(
                (0..self.grid_size.1).map(|new_y| (level + 1, self.grid_size.0 - 1, new_y)),
            );
        } else if x == self.grid_size.0 / 2 && y == self.grid_size.1 / 2 - 1 {
            neighbors.extend((0..self.grid_size.0).map(|new_x| (level + 1, new_x, 0)));
        }

        neighbors
    }
    fn count_adjacent_bugs(&self, level: isize, (x, y): (usize, usize)) -> usize {
        self.get_neighbors(level, (x, y))
            .into_iter()
            .filter(|&(level, x, y)| {
                self.bugs
                    .get(&level)
                    .map(|level_bugs| level_bugs.contains(&(x, y)))
                    .unwrap_or(false)
            })
            .count()
    }
    fn update(&mut self) {
        let (mut min_level, mut max_level) = self.levels;

        let mut new_bugs = HashMap::new();

        for level in (min_level - 1)..(max_level + 2) {
            for (x, y) in iproduct![0..self.grid_size.0, 0..self.grid_size.1] {
                if (x, y) == (self.grid_size.0 / 2, self.grid_size.1 / 2) {
                    continue;
                }

                let is_bug = self
                    .bugs
                    .get(&level)
                    .map(|z| z.contains(&(x, y)))
                    .unwrap_or(false);
                match (is_bug, self.count_adjacent_bugs(level, (x, y))) {
                    (_, 1) | (false, 2) => {
                        min_level = min_level.min(level);
                        max_level = max_level.max(level);
                        new_bugs
                            .entry(level)
                            .or_insert(HashSet::new())
                            .insert((x, y))
                    }
                    _ => true,
                };
            }
        }
        self.bugs = new_bugs;
        self.levels = (min_level, max_level);
    }
    fn count_bugs(&self) -> usize {
        self.bugs.values().flat_map(|level| level.iter()).count()
    }
}

pub fn solve_part_one() -> u64 {
    let input = super::get_input::main(24);
    let mut puzzle = NonRecursivePuzzle::from(&input);
    puzzle.run_until_duplicate()
}

pub fn solve_part_two() -> usize {
    let input = super::get_input::main(24);
    let mut puzzle = RecursivePuzzle::from(&input);
    for _ in 0..200 {
        puzzle.update();
    }
    puzzle.count_bugs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_puzzle() {
        let input = concat!("....#\n", "#..#.\n", "#..##\n", "..#..\n", "#....\n",).to_string();
        let mut puzzle = NonRecursivePuzzle::from(&input);
        assert_eq!(puzzle.bounds, (5, 5));
        assert_eq!(
            puzzle.bug_positions(),
            vec![
                (0, 4),
                (1, 0),
                (1, 3),
                (2, 0),
                (2, 3),
                (2, 4),
                (3, 2),
                (4, 0),
            ]
        );
        for _ in 0..4 {
            puzzle.update();
        }
        assert_eq!(
            puzzle.bug_positions(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 4),
                (2, 0),
                (2, 1),
                (2, 4),
                (4, 0),
                (4, 1)
            ]
        );
    }

    #[test]
    fn test_biodiversity() {
        let puzzle = NonRecursivePuzzle {
            bugs: iproduct![0..5, 0..5]
                .into_iter()
                .map(|(x, y)| (x, y) == (3, 0) || (x, y) == (4, 1))
                .collect(),
            seen_biodiversity: HashSet::new(),
            bounds: (5, 5),
        };
        assert_eq!(puzzle.biodiversity(), 2129920);
    }

    #[test]
    fn test_recursive_neighbors() {
        let puzzle = RecursivePuzzle {
            bugs: HashMap::new(),
            grid_size: (5, 5),
            levels: (0, 0),
        };
        let tests = vec![
            (
                (0, 0, 0),
                vec![(-1, 2, 1), (-1, 1, 2), (0, 0, 1), (0, 1, 0)],
            ),
            (
                (0, 3, 2),
                vec![
                    (1, 4, 0),
                    (1, 4, 1),
                    (1, 4, 2),
                    (1, 4, 3),
                    (1, 4, 4),
                    (0, 3, 1),
                    (0, 3, 3),
                    (0, 4, 2),
                ],
            ),
            ((0, 1, 1), vec![(0, 0, 1), (0, 1, 2), (0, 2, 1), (0, 1, 0)]),
        ];
        for ((level, x, y), result) in tests {
            assert_eq!(
                puzzle
                    .get_neighbors(level, (x, y))
                    .into_iter()
                    .collect::<HashSet<(isize, usize, usize)>>(),
                result.into_iter().collect(),
            )
        }
    }

    #[test]
    fn test_recursive_update() {
        let input = concat!("....#\n", "#..#.\n", "#..##\n", "..#..\n", "#....\n",).to_string();
        let mut puzzle = RecursivePuzzle::from(&input);
        for _ in 0..10 {
            puzzle.update();
        }
        assert_eq!(
            *puzzle.bugs.get(&-5).unwrap(),
            vec![(0, 2), (1, 1), (1, 3), (2, 4), (3, 1), (3, 3), (4, 2)]
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 14539258);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 1977);
    }
}
