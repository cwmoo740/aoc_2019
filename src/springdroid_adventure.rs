use super::intcode::Computer;

fn prepare_program(x: &Vec<&str>) -> Vec<i64> {
    format!("{}\n", x.join("\n"))
        .chars()
        .map(|ch| ch as i64)
        .collect()
}

fn read_output(x: &Vec<i64>) -> (String, i64) {
    let ascii: String = x[0..x.len() - 1]
        .iter()
        .cloned()
        .map(|z| z as u8 as char)
        .collect();
    (ascii, *x.last().unwrap())
}

pub fn solve_part_one() -> i64 {
    let input = prepare_program(&vec![
        "OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "WALK",
    ]);
    let computer = Computer::new(&Computer::load_data(21), &input);
    let output: Vec<i64> = computer.collect();
    let (ascii, hull_damage) = read_output(&output);
    print!("{}", ascii);
    hull_damage
}

pub fn solve_part_two() -> i64 {
    let input: Vec<i64> = prepare_program(&vec![
        "OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "OR E T", "OR H T", "AND T J", "RUN",
    ]);
    let computer = Computer::new(&Computer::load_data(21), &input);
    let output = computer.collect();
    let (ascii, hull_damage) = read_output(&output);
    print!("{}", ascii);
    hull_damage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 19353074);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 1147582556);
    }
}
