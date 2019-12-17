use std::collections::{HashMap, VecDeque};

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
    RelativeBaseOffset,
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
            "09" => OpCode::RelativeBaseOffset,
            "99" => OpCode::Stop,
            z => panic!("opcode failure! {}", z),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<char> for ParameterMode {
    fn from(x: char) -> Self {
        match x {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            _ => panic!("unrecognized parameter mode {}", x),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
    params: (ParameterMode, ParameterMode, ParameterMode),
}

impl From<i64> for Instruction {
    fn from(x: i64) -> Instruction {
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
    pub program: HashMap<i64, i64>,
    pub input_queue: VecDeque<i64>,
    default_input: i64,
    pos: i64,
    relative_base: i64,
}

impl Computer {
    pub fn new(program: Vec<i64>, input: &[i64]) -> Self {
        Computer {
            program: program
                .into_iter()
                .enumerate()
                .fold(
                    HashMap::new(),
                    |mut result, (i, x)| {
                        result.insert(i as i64, x);
                        result
                    },
                ),
            input_queue: input.into_iter().cloned().collect(),
            default_input: 0,
            pos: 0,
            relative_base: 0,
        }
    }
    pub fn load_data(day: usize) -> Vec<i64> {
        super::get_input::main(day)
            .trim()
            .split(",")
            .map(|z| i64::from_str_radix(z, 10).unwrap())
            .collect()
    }
    pub fn get_program(&self) -> Vec<i64> {
        let len = *self.program.keys().max().unwrap();
        let mut result = vec![0; (len + 1) as usize];
        for (&i, &v) in self.program.iter() {
            result[i as usize] = v;
        }
        result
    }
    pub fn set_default_input(&mut self, value: i64) {
        self.default_input = value;
    }
    fn get_instruction(&self) -> Instruction {
        Instruction::from(self.program[&self.pos])
    }
    fn get_value(&mut self, index: i64, mode: ParameterMode) -> i64 {
        let a = *self.program.entry(index).or_insert(0);
        match mode {
            ParameterMode::Position => *self.program.entry(a).or_insert(0),
            ParameterMode::Immediate => a,
            ParameterMode::Relative => *self.program.entry(self.relative_base + a).or_insert(0),
        }
    }
    fn write(&mut self, index: i64, value: i64, mode: ParameterMode) {
        let a = *self.program.entry(index).or_insert(0);
        match mode {
            ParameterMode::Immediate => panic!("immediate mode not supported for writing"),
            ParameterMode::Position => self.program.insert(a, value),
            ParameterMode::Relative => self.program.insert(a + self.relative_base, value)
        };
    }
}

impl<'a> Iterator for Computer {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            let instruction = self.get_instruction();
            match instruction.opcode {
                OpCode::Add => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    self.write(self.pos + 3, a + b, instruction.params.2);
                    self.pos += 4;
                }
                OpCode::Multiply => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    self.write(self.pos + 3, a * b, instruction.params.2);
                    self.pos += 4;
                }
                OpCode::JumpIfTrue => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    if a != 0 {
                        self.pos = self.get_value(self.pos + 2, instruction.params.1);
                    } else {
                        self.pos += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    if a == 0 {
                        self.pos = self.get_value(self.pos + 2, instruction.params.1);
                    } else {
                        self.pos += 3;
                    }
                }
                OpCode::LessThan => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let value = {
                        if a < b {
                            1
                        } else {
                            0
                        }
                    };
                    self.write(self.pos + 3, value, instruction.params.2);
                    self.pos += 4;
                }
                OpCode::Equals => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    let b = self.get_value(self.pos + 2, instruction.params.1);
                    let value = {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    };
                    self.write(self.pos + 3, value, instruction.params.2);
                    self.pos += 4;
                }
                OpCode::Input => {
                    let value = self.input_queue.pop_front().unwrap_or(self.default_input);
                    self.write(self.pos + 1, value, instruction.params.0);
                    self.pos += 2;
                }
                OpCode::Output => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    self.pos += 2;
                    return Some(a);
                }
                OpCode::RelativeBaseOffset => {
                    let a = self.get_value(self.pos + 1, instruction.params.0);
                    self.pos += 2;
                    self.relative_base += a;
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
        let values: Vec<(i64, Instruction)> = vec![
            (1002, Instruction { opcode: OpCode::Multiply, params: (ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position) })
        ];
        for (input, answer) in values {
            assert_eq!(Instruction::from(input), answer);
        }
    }

    #[test]
    fn test_program_with_negative_numbers() {
        let program: Vec<i64> = vec![1101, 100, -1, 4, 0];
        let mut computer = Computer::new(program, &[0]);
        let output = computer.next();
        assert!(output.is_none());
        assert_eq!(computer.get_program(), vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_simple_program() {
        let program: Vec<i64> = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(program, &[0]);
        let output = computer.next();
        assert!(output.is_none());
        assert_eq!(computer.get_program(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_comparisons_with_8() {
        // program, false input, truth input
        let values: Vec<(Vec<i64>, i64, i64)> = vec![
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
        let program: Vec<i64> = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let values: Vec<(i64, i64)> = vec![
            (7, 999),
            (8, 1000),
            (9, 1001),
        ];
        for (val, out) in values {
            let mut computer = Computer::new(program.clone(), &[val]);
            assert_eq!(computer.next().unwrap(), out);
        }
    }

    #[test]
    fn test_relative_mode_parameters() {
        let values: Vec<(Vec<i64>, Vec<i64>)> = vec![
            (vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]),
            (vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], vec![1219070632396864]),
            (vec![104, 1125899906842624, 99], vec![1125899906842624]),
        ];
        for (program, output) in values {
            assert_eq!(Computer::new(program, &[]).collect::<Vec<i64>>(), output);
        }
    }
}