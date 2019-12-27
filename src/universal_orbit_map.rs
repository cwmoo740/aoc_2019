use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    root: String,
    // k: parent, v: children
    planets: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            root: "COM".to_string(),
            planets: HashMap::new(),
        }
    }
    fn add_orbit(&mut self, x: &str) {
        let mut items = x.trim().split(")");
        let parent = items.next().unwrap();
        let child = items.next().unwrap();
        self.planets
            .entry(child.to_string())
            .or_insert(HashSet::new());
        let entry = self
            .planets
            .entry(parent.to_string())
            .or_insert(HashSet::new());
        entry.insert(child.to_string());
    }
    fn make_bidirectional(&mut self) {
        let new_edges: Vec<(String, String)> = self
            .planets
            .iter()
            .flat_map(|(parent, children)| {
                children
                    .iter()
                    .map(move |child| (child.clone(), parent.clone()))
            })
            .collect();

        for (child, parent) in new_edges {
            self.planets
                .entry(child)
                .or_insert(HashSet::new())
                .insert(parent);
        }
    }
    fn bfs(&self, start: &str, dest: &str) -> Option<Vec<String>> {
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(start.to_string());
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let children = self
                .planets
                .get(&current)
                .unwrap()
                .iter()
                .filter(|&z| !parent.contains_key(z))
                .collect::<Vec<&String>>();
            for child in children {
                parent.insert(child.to_string(), current.to_string());
                queue.push_back(child.to_string());
            }
        }
        if !parent.contains_key(dest) {
            return None;
        }
        let mut current = dest;
        let mut result: Vec<String> = vec![dest.to_string()];
        while current != start {
            current = parent.get(current).unwrap();
            result.push(current.to_string());
        }
        result.reverse();
        Some(result)
    }
    fn count_orbits(&self) -> usize {
        let mut orbit_count: HashMap<&str, usize> = HashMap::new();
        orbit_count.insert(&self.root, 0);
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(&self.root);
        while !queue.is_empty() {
            let current_node = queue.pop_front().unwrap();
            let children = self.planets.get(current_node).unwrap();
            for child in children {
                queue.push_back(child);
                orbit_count.insert(child, orbit_count.get(current_node).unwrap() + 1);
            }
        }
        orbit_count.values().sum()
    }
}

impl From<&str> for Graph {
    fn from(x: &str) -> Graph {
        let mut graph = Graph::new();
        for line in x.trim().lines() {
            graph.add_orbit(line);
        }
        graph
    }
}

fn count_orbits(x: &str) -> usize {
    let graph = Graph::from(x);
    graph.count_orbits()
}

fn navigate_to_santa(x: &str) -> usize {
    let mut graph = Graph::from(x);
    graph.make_bidirectional();
    if let Some(path) = graph.bfs("YOU", "SAN") {
        path.len()
    } else {
        0
    }
}

pub fn solve_part_one() -> usize {
    let input = super::get_input::main(6);
    count_orbits(&input)
}

pub fn solve_part_two() -> usize {
    let input = super::get_input::main(6);
    navigate_to_santa(&input) - 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_graph() {
        let input: String = "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L"
            .split(",")
            .collect::<Vec<&str>>()
            .join("\n");
        assert_eq!(count_orbits(&input), 42);
    }

    #[test]
    fn test_make_bidirectional() {
        let input: &str = &"COM)B,B)C,C)D".split(",").collect::<Vec<&str>>().join("\n");
        let mut graph = Graph::from(input);
        graph.make_bidirectional();
        let mut expected_hashmap: HashMap<String, HashSet<String>> = HashMap::new();
        expected_hashmap.insert(
            "COM".to_string(),
            ["B".to_string()].iter().cloned().collect(),
        );
        expected_hashmap.insert(
            "B".to_string(),
            ["COM".to_string(), "C".to_string()]
                .iter()
                .cloned()
                .collect(),
        );
        expected_hashmap.insert(
            "C".to_string(),
            ["B".to_string(), "D".to_string()].iter().cloned().collect(),
        );
        expected_hashmap.insert("D".to_string(), ["C".to_string()].iter().cloned().collect());
        assert_eq!(graph.planets, expected_hashmap);
    }

    #[test]
    fn test_navigation() {
        let input: String = "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L,K)YOU,I)SAN"
            .split(",")
            .collect::<Vec<&str>>()
            .join("\n");
        assert_eq!(navigate_to_santa(&input) - 3, 4);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 224901);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 334);
    }
}
