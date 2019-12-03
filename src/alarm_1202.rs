pub enum OpCode {
    Add,
    Multiply,
    Stop,
}

impl OpCode {
    pub fn from_usize(x: usize) -> Self {
        match x {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Stop,
            z => panic!("opcode failure! {}", z),
        }
    }
}

fn parse_input(x: &String) -> Vec<usize> {
    x
        .split(",")
        .map(|z| z.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn run_opcodes(arr: &mut Vec<usize>, noun: usize, verb: usize) -> () {
    let mut pos: usize = 0;
    arr[1] = noun;
    arr[2] = verb;
    while pos < arr.len() {
        let opcode = OpCode::from_usize(arr[pos]);
        match opcode {
            OpCode::Add => {
                let a = arr[pos + 1];
                let b = arr[pos + 2];
                let c = arr[pos + 3];
                arr[c] = arr[b] + arr[a];
            }
            OpCode::Multiply => {
                let a = arr[pos + 1];
                let b = arr[pos + 2];
                let c = arr[pos + 3];
                arr[c] = arr[b] * arr[a];
            }
            OpCode::Stop => {
                break;
            }
        }
        pos += 4;
    };
}

pub fn solve_part_one() -> usize {
    let mut input = parse_input(&super::get_input::main(2));
    run_opcodes(&mut input, 12, 2);
    *input.get(0).unwrap()
}

pub fn solve_part_two() -> Option<usize> {
    let input = parse_input(&super::get_input::main(2));
    for noun in 0usize..100usize {
        for verb in 0usize..100usize {
            let mut cloned = input.clone();
            run_opcodes(&mut cloned, noun, verb);
            if cloned[0] == 19690720 {
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
    fn test_parse_input() {
        let input_str = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        assert_eq!(parse_input(&input_str), vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_run_opcodes() {
        let cases: Vec<(Vec<usize>, Vec<usize>)> = vec![
            (vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
        ];
        for (mut arr, expected) in cases {
            run_opcodes(&mut arr, 12, 2);
            assert_eq!(arr, expected);
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