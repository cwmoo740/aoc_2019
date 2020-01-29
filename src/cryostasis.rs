#[cfg(test)]
use itertools::Itertools;

use super::intcode::Computer;

fn get_sand() -> String {
    concat!("north,", "north,", "take sand,", "south,", "south,",).to_string()
}

fn get_space_heater() -> String {
    concat!("south,", "take space heater,", "north,").to_string()
}

fn get_wreath() -> String {
    concat!("south,", "west,", "take wreath,", "east,", "north,").to_string()
}

fn get_space_law_brochure() -> String {
    concat!(
        "south,",
        "west,",
        "south,",
        "take space law space brochure,",
        "north,",
        "east,",
        "north,"
    )
    .to_string()
}

fn get_pointer() -> String {
    concat!(
        "south,",
        "west,",
        "south,",
        "south,",
        "take pointer,",
        "north,",
        "north,",
        "east,",
        "north,"
    )
    .to_string()
}

fn get_planetoid() -> String {
    concat!("west,", "south,", "take planetoid,", "north,", "east,").to_string()
}

fn get_loom() -> String {
    concat!(
        "south,",
        "south,",
        "east,",
        "take loom,",
        "west,",
        "north,",
        "north,"
    )
    .to_string()
}

fn get_festive_hat() -> String {
    concat!("west,", "west,", "take festive hat,", "east,", "east,").to_string()
}

fn bring_all_to_checkpoint() -> String {
    vec![
        get_sand(),
        get_space_heater(),
        get_wreath(),
        get_space_law_brochure(),
        get_pointer(),
        get_planetoid(),
        get_loom(),
        get_festive_hat(),
        concat!(
            "west,",
            "west,",
            "south,",
            "west,",
            "drop sand,",
            "drop space heater,",
            "drop wreath,",
            "drop space law space brochure,",
            "drop planetoid,",
            "drop loom,",
            "drop festive hat,",
        )
        .to_string(),
    ]
    .join("")
}

#[cfg(test)]
fn take_items(items: &Vec<&str>) -> Vec<String> {
    items.iter().map(|z| format!("take {}", z)).collect()
}

#[cfg(test)]
fn drop_items(items: &Vec<&str>) -> Vec<String> {
    items.iter().map(|z| format!("drop {}", z)).collect()
}

#[cfg(test)]
fn find_item_combination(computer: &mut Computer) -> Vec<&str> {
    let ascii = bring_all_to_checkpoint();
    let input: Vec<&str> = ascii.split(",").filter(|z| !z.is_empty()).collect();
    computer.add_input(&Computer::ascii_to_intcode(&input));
    computer.last();
    let all_items = vec![
        "sand",
        "space heater",
        "wreath",
        "space law space brochure",
        "planetoid",
        "loom",
        "festive hat",
    ];

    for k in 1..all_items.len() {
        for combination in all_items.clone().into_iter().combinations(k) {
            computer.add_input(&Computer::ascii_to_intcode(
                &take_items(&combination)
                    .iter()
                    .map(String::as_str)
                    .collect(),
            ));
            computer.add_input(&Computer::ascii_to_intcode(&vec!["north"]));
            if Computer::intcode_to_ascii(&computer.collect()).contains("get in by typing") {
                return combination;
            }
            computer.add_input(&Computer::ascii_to_intcode(
                &drop_items(&combination)
                    .iter()
                    .map(String::as_str)
                    .collect(),
            ));
        }
    }
    Vec::new()
}

pub fn solve_part_one() -> String {
    let ascii = bring_all_to_checkpoint();
    let mut input: Vec<&str> = ascii.split(",").filter(|z| !z.is_empty()).collect();
    input.append(&mut vec![
        "take sand",
        "take wreath",
        "take planetoid",
        "north",
    ]);
    let mut computer: Computer = Computer::new(
        &Computer::load_data(25),
        &Computer::ascii_to_intcode(&input),
    );
    computer.yield_on_empty = true;
    let output = Computer::intcode_to_ascii(&computer.collect());
    output
        .lines()
        .skip_while(|&line| !line.contains("get in by typing"))
        .flat_map(|line| line.chars())
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_item_combination() {
        let mut computer: Computer = Computer::new(&Computer::load_data(25), &[]);
        computer.yield_on_empty = true;
        assert_eq!(
            find_item_combination(&mut computer),
            vec!["sand", "wreath", "planetoid"]
        );
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), "529920".to_string());
    }
}
