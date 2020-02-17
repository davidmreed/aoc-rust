use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::collections::HashMap;

type Opcode = i32;

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

    fn execute(&mut self) -> i32 {
        loop {
            match self.get_direct(self.ip) {
                1 => {
                    self.mem.insert(
                        self.get_direct(self.ip + 3),
                        self.get_indirect(self.ip + 1) + self.get_indirect(self.ip + 2)
                    );
                },
                2 => {
                    self.mem.insert(
                        self.get_direct(self.ip + 3),
                        self.get_indirect(self.ip + 1) * self.get_indirect(self.ip + 2)
                    );
                },
                99 => return self.get_direct(0), 
                _ => panic!("Invalid opcode"),
            }
            self.ip += 4
        }
    }
}

fn main() {
    let mut machine = IntMachine::from_file("input.txt");

    println!("Part 1 result: {}", machine.execute());

    'noun: for noun in 0..99 {
        for verb in 0..99 {
            let mut machine = IntMachine::from_file("input.txt");
            machine.mem.insert(1, noun);
            machine.mem.insert(2, verb);

            if machine.execute() == 19690720 {
                println!("Part 2 result: noun {}, verb {}", noun, verb);
                break 'noun;
            }
        }
    }
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
}

