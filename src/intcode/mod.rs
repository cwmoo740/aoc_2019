use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
pub enum OpCode {
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
pub enum ParameterMode {
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
pub struct Instruction {
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

#[derive(Default, Debug)]
pub struct Computer {
    pub program: Vec<isize>,
    pub input_queue: VecDeque<isize>,
    pos: usize,
}

impl Computer {
    pub fn new(program: Vec<isize>, input: &[isize]) -> Self {
        Computer {
            program,
            input_queue: input.into_iter().cloned().collect(),
            pos: 0,
        }
    }
    pub fn get_program(day: usize) -> Vec<isize> {
        super::get_input::main(day)
            .trim()
            .split(",")
            .map(|z| isize::from_str_radix(z, 10).unwrap())
            .collect()
    }
    fn get_value(&self, index: usize, mode: ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => self.program[self.program[index] as usize],
            ParameterMode::Immediate => self.program[index],
        }
    }
}

impl<'a> Iterator for Computer {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        loop {
            let instruction = Instruction::from(self.program[self.pos]);
            match instruction.opcode {
                OpCode::Add => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let c = self.program[self.pos + 3] as usize;
                    self.program[c] = a + b;
                    self.pos += 4;
                }
                OpCode::Multiply => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let c = self.program[self.pos + 3] as usize;
                    self.program[c] = b * a;
                    self.pos += 4;
                }
                OpCode::JumpIfTrue => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    if a != 0 {
                        self.pos = self.get_value(self.pos + 2, instruction.params.1) as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    if a == 0 {
                        self.pos = self.get_value(self.pos + 2, instruction.params.1) as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                OpCode::LessThan => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let c = self.program[self.pos + 3] as usize;
                    if a < b {
                        self.program[c] = 1;
                    } else {
                        self.program[c] = 0;
                    }
                    self.pos += 4;
                }
                OpCode::Equals => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let c = self.program[self.pos + 3] as usize;
                    if a == b {
                        self.program[c] = 1;
                    } else {
                        self.program[c] = 0;
                    }
                    self.pos += 4;
                }
                OpCode::Input => {
                    let a = self.program[self.pos + 1] as usize;
                    self.program[a] = self.input_queue.pop_front().unwrap();
                    self.pos += 2;
                }
                OpCode::Output => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    self.pos += 2;
                    return Some(a);
                }
                OpCode::Stop => {
                    return None;
                }
            }
        };
    }
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
        let program: Vec<isize> = vec![1101, 100, -1, 4, 0];
        let mut computer = Computer::new(program, &[0]);
        let output = computer.next();
        assert!(output.is_none());
        assert_eq!(computer.program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_simple_program() {
        let program: Vec<isize> = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(program, &[0]);
        let output = computer.next();
        assert!(output.is_none());
        assert_eq!(computer.program, vec![1002, 4, 3, 4, 99]);
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
            let mut computer = Computer::new(program.clone(), &[falsy]);
            assert_eq!(computer.next().unwrap(), 0);
            computer = Computer::new(program.clone(), &[truthy]);
            assert_eq!(computer.next().unwrap(), 1);
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
            let mut computer = Computer::new(program.clone(), &[val]);
            assert_eq!(computer.next().unwrap(), out);
        }
    }
}