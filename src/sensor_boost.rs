use super::intcode::Computer;

pub fn solve_part_one() -> Vec<i64> {
    let program = Computer::load_data(9);
    let computer = Computer::new(&program, &[1]);
    computer.collect()
}

pub fn solve_part_two() -> Vec<i64> {
    let program = Computer::load_data(9);
    let computer = Computer::new(&program, &[2]);
    computer.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), vec![3409270027]);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), vec![82760]);
    }
}
