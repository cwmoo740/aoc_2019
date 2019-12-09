#[derive(Debug, Eq, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl From<&str> for OpCode {
    fn from(x: &str) -> Self {
        match x {
            "01" => OpCode::Add,
            "02" => OpCode::Multiply,
            "03" => OpCode::Input,
            "04" => OpCode::Output,
            "05" => OpCode::JumpIfTrue,
            "06" => OpCode::JumpIfFalse,
            "07" => OpCode::LessThan,
            "08" => OpCode::Equals,
            "99" => OpCode::Stop,
            z => panic!("opcode failure! {}", z),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<char> for ParameterMode {
    fn from(x: char) -> Self {
        match x {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => panic!("unrecognized parameter mode {}", x),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    opcode: OpCode,
    params: (ParameterMode, ParameterMode, ParameterMode),
}

impl From<isize> for Instruction {
    fn from(x: isize) -> Instruction {
        let s = format!("{:05}", x);
        let opcode = OpCode::from(&s[3..]);
        let mut chars = s.chars();
        let a = ParameterMode::from(chars.next().unwrap());
        let b = ParameterMode::from(chars.next().unwrap());
        let c = ParameterMode::from(chars.next().unwrap());
        let params = (
            c,
            b,
            a,
        );
        Instruction {
            opcode,
            params,
        }
    }
}

fn get_value(arr: &Vec<isize>, index: usize, mode: ParameterMode) -> isize {
    match mode {
        ParameterMode::Position => arr[arr[index] as usize],
        ParameterMode::Immediate => arr[index],
    }
}

fn run_instructions(arr: &mut Vec<isize>, input: isize) -> Vec<isize> {
    let mut pos: usize = 0;
    let mut output: Vec<isize> = Vec::new();
    while pos < arr.len() {
        let instruction = Instruction::from(arr[pos]);
        match instruction.opcode {
            OpCode::Add => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                let b = get_value(arr, pos + 2, instruction.params.1);
                let c = arr[pos + 3] as usize;
                arr[c] = a + b;
                pos += 4;
            }
            OpCode::Multiply => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                let b = get_value(arr, pos + 2, instruction.params.1);
                let c = arr[pos + 3] as usize;
                arr[c] = b * a;
                pos += 4;
            }
            OpCode::JumpIfTrue => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                if a != 0 {
                    pos = get_value(arr, pos + 2, instruction.params.1) as usize;
                } else {
                    pos += 3;
                }
            }
            OpCode::JumpIfFalse => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                if a == 0 {
                    pos = get_value(arr, pos + 2, instruction.params.1) as usize;
                } else {
                    pos += 3;
                }
            }
            OpCode::LessThan => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                let b = get_value(arr, pos + 2, instruction.params.1);
                let c = arr[pos + 3] as usize;
                if a < b {
                    arr[c] = 1;
                } else {
                    arr[c] = 0;
                }
                pos += 4;
            }
            OpCode::Equals => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                let b = get_value(arr, pos + 2, instruction.params.1);
                let c = arr[pos + 3] as usize;
                if a == b {
                    arr[c] = 1;
                } else {
                    arr[c] = 0;
                }
                pos += 4;
            }
            OpCode::Input => {
                let a = arr[pos + 1] as usize;
                arr[a] = input;
                pos += 2;
            }
            OpCode::Output => {
                let a = get_value(arr, pos + 1, instruction.params.0);
                output.push(a);
                pos += 2;
            }
            OpCode::Stop => {
                break;
            }
        }
    };
    output
}

fn get_program() -> Vec<isize> {
    super::get_input::main(5)
        .trim()
        .split(",")
        .map(|z| isize::from_str_radix(z, 10).unwrap())
        .collect()
}

pub fn solve_part_one() -> Vec<isize> {
    let mut program = get_program();
    run_instructions(&mut program, 1)
}

pub fn solve_part_two() -> Vec<isize> {
    let mut program = get_program();
    run_instructions(&mut program, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let values: Vec<(isize, Instruction)> = vec![
            (1002, Instruction { opcode: OpCode::Multiply, params: (ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position) })
        ];
        for (input, answer) in values {
            assert_eq!(Instruction::from(input), answer);
        }
    }

    #[test]
    fn test_program_with_negative_numbers() {
        let mut program: Vec<isize> = vec![1101, 100, -1, 4, 0];
        run_instructions(&mut program, 0);
        assert_eq!(program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_simple_program() {
        let mut program: Vec<isize> = vec![1002, 4, 3, 4, 33];
        run_instructions(&mut program, 0);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_comparisons_with_8() {
        // program, false input, truth input
        let values: Vec<(Vec<isize>, isize, isize)> = vec![
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 2, 8), // equal to 8
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9, 4), // less than 8
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9, 8), // equal to 8
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 7), // less than 8
        ];
        for (program, falsy, truthy) in values {
            let mut cloned = program.clone();
            assert_eq!(run_instructions(&mut cloned, falsy), vec![0]);
            cloned = program.clone();
            assert_eq!(run_instructions(&mut cloned, truthy), vec![1]);
        }
    }

    #[test]
    fn test_complex_program() {
        let program: Vec<isize> = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let values: Vec<(isize, isize)> = vec![
            (7, 999),
            (8, 1000),
            (9, 1001),
        ];
        for (val, out) in values {
            let mut cloned = program.clone();
            assert_eq!(run_instructions(&mut cloned, val), vec![out]);
        }
    }

    #[test]
    fn test_solve_part_one() {
        let output = solve_part_one();
        assert!(output.iter().rev().skip(1).all(|x| *x == 0));
        assert_eq!(*output.last().unwrap(), 9431221);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(), vec![1409363]);
    }
}
