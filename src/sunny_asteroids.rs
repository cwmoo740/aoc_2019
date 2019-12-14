use super::intcode::Computer;

pub fn solve_part_one() -> Vec<isize> {
    let program = Computer::load_data(5);
    let computer = Computer::new(program, &[1]);
    computer.map(|elem| elem as isize).collect()
}

pub fn solve_part_two() -> isize {
    let program = Computer::load_data(5);
    let computer = Computer::new(program, &[5]);
    computer.into_iter().last().unwrap() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let output = solve_part_one();
        assert!(output.iter().rev().skip(1).all(|x| *x == 0));
        assert_eq!(*output.last().unwrap(), 9431221);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 1409363);
    }
}
