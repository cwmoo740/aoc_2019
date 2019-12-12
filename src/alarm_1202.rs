use super::intcode::Computer;

pub fn solve_part_one() -> isize {
    let program = {
        let mut x = Computer::get_program(2);
        x[1] = 12;
        x[2] = 2;
        x
    };
    let mut computer = Computer::new(program, &[]);
    computer.next();
    computer.program[0]
}

pub fn solve_part_two() -> Option<isize> {
    let program = Computer::get_program(2);
    for noun in 0isize..100isize {
        for verb in 0isize..100isize {
            let program = {
                let mut cloned = program.clone();
                cloned[1] = noun;
                cloned[2] = verb;
                cloned
            };
            let mut computer = Computer::new(program, &[]);
            computer.next();
            if computer.program[0] == 19690720 {
                return Option::Some(100 * noun + verb);
            }
        }
    }
    Option::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_opcodes() {
        let cases: Vec<(Vec<isize>, Vec<isize>)> = vec![
            (vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
        ];
        for (program, expected) in cases {
            let mut computer = Computer::new(program, &[]);
            computer.next();
            assert_eq!(computer.program, expected);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 4090689);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two().unwrap(), 100 * 77 + 33);
    }
}