use std::fs::File;
use std::io::{Read, stdin};
use std::convert::TryInto;
use std::collections::HashMap;

type Opcode = i32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Parameter {
    LEFT,
    RIGHT,
    TARGET,
}

fn get_parameter_mode(op: Opcode, parameter: Parameter) -> ParameterMode {
    let mode = match parameter {
        Parameter::LEFT => (op % 1000) > 100,
        Parameter::RIGHT => (op % 10000) > 1000,
        Parameter::TARGET => false
    };

    if mode {
        ParameterMode::IMMEDIATE
    } else {
        ParameterMode::POSITION
    }
}

fn get_opcode_value(op: Opcode) -> Opcode {
    op % 100
}

#[derive(Debug)]
#[derive(PartialEq)]
struct IntMachine {
    mem: HashMap<i32, Opcode>,
    ip: i32,
}

impl IntMachine {
    fn new() -> IntMachine {
        IntMachine{ mem: HashMap::new(), ip: 0 }
    }

    fn from_string(s: &str) -> IntMachine {
        let mut machine = IntMachine::new();

        for (index, op) in s.split(',')
            .map(|x| x.trim().parse::<i32>()
                .expect(format!("{} is not a valid opcode", x).as_str()))
            .enumerate() {
            machine.mem.insert(index.try_into().unwrap(), op);
        }
        machine
    }

    fn from_file(file_name: &str) -> IntMachine {
        let mut file = File::open(file_name).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        IntMachine::from_string(&content)
    }

    fn get_direct(&self, index: i32) -> i32 {
        *self.mem.get(&index).unwrap()
    }

    fn get_indirect(&self, index: i32) -> i32 {
        self.get_direct(self.get_direct(index))
    }

    fn get_parameter_index(&self, param: Parameter) -> i32 {
        match param {
            Parameter::LEFT => self.ip + 1,
            Parameter::RIGHT => self.ip + 2,
            Parameter::TARGET => self.ip + 3
        }
    }
    
    fn get(&self, op: Opcode, param: Parameter) -> i32 {
        let mode = get_parameter_mode(op, param);
        let index = self.get_parameter_index(param);

        match mode {
            ParameterMode::IMMEDIATE => self.get_direct(index),
            ParameterMode::POSITION => self.get_indirect(index)
        }
    }

    fn get_target(&self, op: Opcode, param: Parameter) -> i32 {
        self.get_direct(self.get_parameter_index(param))
    }
    
    fn execute(&mut self) -> i32 {
        loop {
            let op = self.get_direct(self.ip);

            match get_opcode_value(op) {
                1 => {
                    self.mem.insert(
                        self.get_target(op, Parameter::TARGET),
                        self.get(op, Parameter::LEFT) + self.get(op, Parameter::RIGHT)
                    );
                    self.ip += 4;
                },
                2 => {
                    self.mem.insert(
                        self.get_target(op, Parameter::TARGET),
                        self.get(op, Parameter::LEFT)  * self.get(op, Parameter::RIGHT)
                    );
                    self.ip += 4;
                },
                3 => {
                    let mut input = String::new();
                    println!("Enter input: ");
                    stdin().read_line(&mut input).unwrap();
                    self.mem.insert(
                        self.get_target(op, Parameter::LEFT),
                        input.trim().parse().expect("Please enter a number")
                    );
                    self.ip += 2;
                },
                4 => {
                    println!(
                        "{}",
                        self.get(op, Parameter::LEFT)
                    );
                    self.ip += 2;
                }
                99 => return self.get_direct(0), 
                _ => panic!(format!("Invalid opcode {}", op)),
            }
        }
    }
}

fn main() {
    let mut machine = IntMachine::from_file("input.txt");

    println!("Part 1 result: {}", machine.execute());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let map: IntMachine = IntMachine{mem:[(0, 1),(1, 9), (2, 0), (3, 99), (4, 30), (5, 40), (6, 50)].iter().cloned().collect(), ip: 0 }; 
        assert_eq!(
            IntMachine::from_string("1,9,0,99,30,40,50"),
            map
        );
    }

    #[test]
    fn test_get_direct() {
        assert_eq!(
            IntMachine::from_string("1,2,3").get_direct(1),
            2
        );
    }

    #[test]
    fn test_get_indirect() {
        assert_eq!(
            IntMachine::from_string("1,2,3").get_indirect(1),
            3
        );
    }

    #[test]
    fn test_execute_addition() {
        assert_eq!(
            IntMachine::from_string("1,0,0,0,99").execute(),
            2
        );
    }
    
    #[test]
    fn test_execute_multiplication() {
        assert_eq!(
            IntMachine::from_string("2,0,5,0,99,3").execute(),
            6
        );
    }

    #[test]
    fn test_execute_program() {
        assert_eq!(
            IntMachine::from_string("1,1,1,4,99,5,6,0,99").execute(),
            30 
        );
    }

    #[test]
    fn test_execute_program_indirect() {
        assert_eq!(
            IntMachine::from_string("1002,4,3,4,33").execute(),
            1002
        );
    }

    #[test]
    fn test_get_parameter_mode() {
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(02, Parameter::LEFT)
        );
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(02, Parameter::RIGHT)
        );
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(02, Parameter::TARGET)
        );
        assert_eq!(
            ParameterMode::IMMEDIATE,
            get_parameter_mode(102, Parameter::LEFT)
        );
        assert_eq!(
            ParameterMode::IMMEDIATE,
            get_parameter_mode(1002, Parameter::RIGHT)
        );
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(10002, Parameter::TARGET)
        );
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(10002, Parameter::LEFT)
        );
        assert_eq!(
            ParameterMode::POSITION,
            get_parameter_mode(10002, Parameter::RIGHT)
        );
    }

    #[test]
    fn test_get_opcode_value() {
        assert_eq!(2, get_opcode_value(1102));
        assert_eq!(1, get_opcode_value(1));
        assert_eq!(99, get_opcode_value(1099));
    }

    #[test]
    fn test_get() {
        let machine = IntMachine::from_string("1002,4,3,4,33");
        let opcode = machine.get_direct(0);
        
        assert_eq!(1002, opcode);
        assert_eq!(
            33,
            machine.get(opcode, Parameter::LEFT)
        );
        assert_eq!(
            3,
            machine.get(opcode, Parameter::RIGHT)
        );
    }
}

