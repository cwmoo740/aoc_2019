use std::collections::HashSet;

use super::intcode::Computer;

fn all_unique_settings(x: &Vec<i64>) -> bool {
    let items: HashSet<i64> = x.into_iter().cloned().collect();
    items.len() == x.len()
}

fn run_amplifiers(program: &Vec<i64>, settings: &Vec<i64>) -> i64 {
    let mut output: Vec<i64> = vec![0];
    for &setting in settings {
        let input = vec![setting, *output.first().unwrap()];
        let computer = Computer::new(program.clone(), &input);
        output = computer.collect();
    }
    *output.first().unwrap()
}

fn run_amplifiers_with_feedback(program: &Vec<i64>, settings: &Vec<i64>) -> i64 {
    let mut computers: Vec<Computer> = settings
        .iter()
        .map(|&setting| Computer::new(program.clone(), &[setting]))
        .collect();
    let mut last_output: i64 = 0;
    'outer: loop {
        for computer in computers.iter_mut() {
            computer.input_queue.push_back(last_output);
            if let Some(output) = computer.next() {
                last_output = output;
            } else {
                break 'outer;
            }
        }
    }
    last_output
}

fn maximum_input_combination(program: &Vec<i64>) -> i64 {
    iproduct!(0..5, 0..5, 0..5, 0..5, 0..5)
        .into_iter()
        .map(|(a, b, c, d, e)| vec![a, b, c, d, e])
        .filter(all_unique_settings)
        .map(|settings| run_amplifiers(&program, &settings))
        .max()
        .unwrap()
}

fn maximum_input_combination_feedback(program: &Vec<i64>) -> i64 {
    iproduct!(5..10, 5..10, 5..10, 5..10, 5..10)
        .into_iter()
        .map(|(a, b, c, d, e)| vec![a, b, c, d, e])
        .filter(all_unique_settings)
        .map(|settings| run_amplifiers_with_feedback(&program, &settings))
        .max()
        .unwrap()
}

pub fn solve_part_one() -> i64 {
    let program = Computer::load_data(7);
    maximum_input_combination(&program)
}

pub fn solve_part_two() -> i64 {
    let program = Computer::load_data(7);
    maximum_input_combination_feedback(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_input_combination() {
        let values: Vec<(Vec<i64>, i64)> = vec![
            (
                vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ],
                43210,
            ),
            (
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0,
                ],
                54321,
            ),
            (
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ],
                65210,
            ),
        ];
        for (program, output) in values {
            assert_eq!(maximum_input_combination(&program), output);
        }
    }

    #[test]
    fn test_run_amplifiers_with_feedback() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let settings = vec![9, 8, 7, 6, 5];
        assert_eq!(run_amplifiers_with_feedback(&program, &settings), 139629729);
    }

    #[test]
    fn test_maximum_input_combination_feedback() {
        let values: Vec<(Vec<i64>, i64)> = vec![
            (
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                139629729,
            ),
            (
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                18216,
            ),
        ];
        for (program, output) in values {
            assert_eq!(maximum_input_combination_feedback(&program), output);
        }
    }

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(), 21860);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), 2645740);
    }
}
