use super::intcode::Computer;

pub fn solve_part_one() -> i64 {
    let input = Computer::ascii_to_intcode(&vec![
        "OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "WALK",
    ]);
    let computer = Computer::new(&Computer::load_data(21), &input);
    let output: Vec<i64> = computer.collect();
    let ascii = Computer::intcode_to_ascii(&output);
    print!("{}", ascii);
    *output.last().unwrap()
}

pub fn solve_part_two() -> i64 {
    let input: Vec<i64> = Computer::ascii_to_intcode(&vec![
        "OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "OR E T", "OR H T", "AND T J", "RUN",
    ]);
    let computer = Computer::new(&Computer::load_data(21), &input);
    let output = computer.collect();
    let ascii = Computer::intcode_to_ascii(&output);
    print!("{}", ascii);
    *output.last().unwrap()
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
