use std::collections::{HashMap, VecDeque};

type Reactions = HashMap<String, (usize, Vec<(usize, String)>)>;

fn parse_chemical(line: &str) -> (usize, String) {
    let mut split = line.trim().split(" ");
    let quantity: usize = split.next().unwrap().parse().unwrap();
    let chemical: &str = split.next().unwrap();
    (quantity, chemical.to_string())
}

fn parse_input(input: String) -> Reactions {
    let mut reactions: Reactions = HashMap::new();
    for line in input.trim().lines() {
        let mut split = line.trim().split(" => ");
        let ingredients = split
            .next()
            .unwrap()
            .split(", ")
            .map(parse_chemical)
            .collect::<Vec<(usize, String)>>();
        let (quantity, chemical) = parse_chemical(split.next().unwrap());
        reactions.insert(chemical.to_string(), (quantity, ingredients));
    }
    reactions
}

fn get_required_ore(fuel_quantity: usize, reactions: &Reactions) -> usize {
    let mut ingredients: HashMap<&str, usize> = HashMap::new();
    let mut required = VecDeque::new();
    required.push_back(("FUEL", fuel_quantity));

    let mut ore_used = 0;
    while !required.is_empty() {
        match required.pop_front() {
            Some(("ORE", mut quantity)) => {
                let extra_ore = ingredients.entry("ORE").or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra_ore);
                quantity -= extra_used;
                *extra_ore -= extra_used;
                ore_used += quantity;
            }
            Some((needed_chemical, mut quantity)) => {
                let extra_chemical = ingredients.entry(needed_chemical).or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra_chemical);
                quantity -= extra_used;
                *extra_chemical -= extra_used;
                if quantity > 0 {
                    let (reaction_quantity, reagents) = reactions.get(needed_chemical).unwrap();
                    let reaction_multiplier = ((quantity - 1) / *reaction_quantity) + 1;
                    *extra_chemical = reaction_quantity * reaction_multiplier - quantity;
                    for (reagent_quantity, reagent_chemical) in reagents {
                        required
                            .push_back((reagent_chemical, reagent_quantity * reaction_multiplier));
                    }
                }
            }
            _ => (),
        }
    }
    ore_used
}

fn bisect(min: usize, max: usize) -> usize {
    ((max as f64 / 2f64) + (min as f64 / 2f64)).floor() as usize
}

fn binary_search(available_ore: usize, reactions: &Reactions) -> usize {
    let mut max_bound = available_ore;
    let mut min_bound = 0usize;
    let mut current_value = bisect(min_bound, max_bound);
    loop {
        let required_ore = get_required_ore(current_value, &reactions);
        if required_ore < available_ore {
            min_bound = current_value;
        } else if required_ore > available_ore {
            max_bound = current_value;
        }
        if max_bound == min_bound || max_bound - min_bound == 1 {
            break;
        }
        current_value = bisect(min_bound, max_bound);
    }
    current_value
}

pub fn solve_part_one() -> usize {
    get_required_ore(1, &parse_input(super::get_input::main(14)))
}

pub fn solve_part_two() -> usize {
    let max_ore = 1000000000000usize;
    let reactions = parse_input(super::get_input::main(14));
    binary_search(max_ore, &reactions)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> String {
        String::from(
            "10 ORE => 10 A
               1 ORE => 1 B
               7 A, 1 B => 1 C
               7 A, 1 C => 1 D
               7 A, 1 D => 1 E
               7 A, 1 E => 1 FUEL",
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(get_test_input()), {
            let mut reactions: Reactions = HashMap::new();
            let entries: Vec<(&str, (usize, Vec<(usize, &str)>))> = vec![
                ("A", (10, vec![(10, "ORE")])),
                ("B", (1, vec![(1, "ORE")])),
                ("C", (1, vec![(7, "A"), (1, "B")])),
                ("D", (1, vec![(7, "A"), (1, "C")])),
                ("E", (1, vec![(7, "A"), (1, "D")])),
                ("FUEL", (1, vec![(7, "A"), (1, "E")])),
            ];
            for (key, (quantity, val)) in entries {
                reactions.insert(
                    key.to_string(),
                    (
                        quantity,
                        val.into_iter()
                            .map(|(q, reagent)| (q, reagent.to_string()))
                            .collect(),
                    ),
                );
            }
            reactions
        }, )
    }

    #[test]
    fn test_produce_fuel() {
        assert_eq!(get_required_ore(1, &parse_input(get_test_input())), 31, )
    }

    #[test]
    fn test_bisect() {
        assert_eq!(
            bisect(usize::min_value(), usize::max_value()),
            usize::max_value() / 2 + 1,
        );
    }

    #[test]
    fn test_binary_search() {
        let input = "
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        "
            .to_string();
        assert_eq!(binary_search(1000000000000, &parse_input(input)), 82892753);
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 346961);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 4065790);
    }
}
