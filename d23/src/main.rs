use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug)]
enum InstructionType {
    Hlf,
    Tpl,
    Inc,
    Jmp,
    Jie,
    Jio,
}

impl From<&str> for InstructionType {
    fn from(value: &str) -> Self {
        match value {
            "hlf" => InstructionType::Hlf,
            "tpl" => InstructionType::Tpl,
            "inc" => InstructionType::Inc,
            "jmp" => InstructionType::Jmp,
            "jie" => InstructionType::Jie,
            "jio" => InstructionType::Jio,
            _ => panic!("Should not be here"),
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
    None,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" | "a," => Register::A,
            "b" | "b," => Register::B,
            _ => Register::None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    register: Register,
    offset: i64,
}
impl Instruction {
    pub(crate) fn new_from(line: String) -> Self {
        let split: Vec<&str> = line.split(' ').collect();

        let instruction_type = split[0].into();
        let register = split[1].into();
        let offset = split.last().unwrap().parse().unwrap_or(i64::MIN);

        Self {
            instruction_type,
            register,
            offset,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut instruction_list = Vec::new();
    let mut register_a = 0;
    let mut register_b = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let instruction = Instruction::new_from(line);
        instruction_list.push(instruction);
    }

    // for instruction in &instruction_list {
    //     println!("{instruction:?}");
    // }

    // Solve
    let mut pos: i64 = 0;
    while pos >= 0 && pos < instruction_list.len() as i64 {
        let instruction = instruction_list.get(pos as usize).unwrap();
        match instruction.instruction_type {
            InstructionType::Hlf => match instruction.register {
                Register::A => register_a /= 2,
                Register::B => register_b /= 2,
                Register::None => panic!("Should not be here!"),
            },
            InstructionType::Tpl => match instruction.register {
                Register::A => register_a *= 3,
                Register::B => register_b *= 3,
                Register::None => panic!("Should not be here!"),
            },
            InstructionType::Inc => match instruction.register {
                Register::A => register_a += 1,
                Register::B => register_b += 1,
                Register::None => panic!("Should not be here!"),
            },
            InstructionType::Jmp => {
                pos += instruction.offset;
                continue;
            }
            InstructionType::Jie => match instruction.register {
                Register::A => {
                    if register_a % 2 == 0 {
                        pos += instruction.offset;
                        continue;
                    }
                }
                Register::B => {
                    if register_b % 2 == 0 {
                        pos += instruction.offset;
                        continue;
                    }
                }
                Register::None => panic!("Should not be here!"),
            },
            InstructionType::Jio => match instruction.register {
                Register::A => {
                    if register_a == 1 {
                        pos += instruction.offset;
                        continue;
                    }
                }
                Register::B => {
                    if register_b == 1 {
                        pos += instruction.offset;
                        continue;
                    }
                }
                Register::None => panic!("Should not be here!"),
            },
        }
        pos += 1;
    }

    // Result
    println!("Result is {}", register_b);
}

fn run2(input_file: &str) {
     // Preamble
     let mut instruction_list = Vec::new();
     let mut register_a:i64 = 1;
     let mut register_b:i64 = 0;
 
     // Parse
     let file = File::open(input_file).unwrap();
     let reader = BufReader::new(file);
 
     for line in reader.lines() {
         let line = line.unwrap().trim().to_string();
         let instruction = Instruction::new_from(line);
         instruction_list.push(instruction);
     }
 
    //  for instruction in &instruction_list {
    //      println!("{instruction:?}");
    //  }
 
     // Solve
     let mut pos: i64 = 0;
     while pos >= 0 && pos < instruction_list.len() as i64 {
         let instruction = instruction_list.get(pos as usize).unwrap();
         match instruction.instruction_type {
             InstructionType::Hlf => match instruction.register {
                 Register::A => register_a /= 2,
                 Register::B => register_b /= 2,
                 Register::None => panic!("Should not be here!"),
             },
             InstructionType::Tpl => match instruction.register {
                 Register::A => register_a *= 3,
                 Register::B => register_b *= 3,
                 Register::None => panic!("Should not be here!"),
             },
             InstructionType::Inc => match instruction.register {
                 Register::A => register_a += 1,
                 Register::B => register_b += 1,
                 Register::None => panic!("Should not be here!"),
             },
             InstructionType::Jmp => {
                 pos += instruction.offset;
                 continue;
             }
             InstructionType::Jie => match instruction.register {
                 Register::A => {
                     if register_a % 2 == 0 {
                         pos += instruction.offset;
                         continue;
                     }
                 }
                 Register::B => {
                     if register_b % 2 == 0 {
                         pos += instruction.offset;
                         continue;
                     }
                 }
                 Register::None => panic!("Should not be here!"),
             },
             InstructionType::Jio => match instruction.register {
                 Register::A => {
                     if register_a == 1 {
                         pos += instruction.offset;
                         continue;
                     }
                 }
                 Register::B => {
                     if register_b == 1 {
                         pos += instruction.offset;
                         continue;
                     }
                 }
                 Register::None => panic!("Should not be here!"),
             },
         }
         pos += 1;
     }
 
     // Result
     println!("Result is {}", register_b);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
