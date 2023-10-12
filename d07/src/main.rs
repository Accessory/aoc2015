use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use regex::Regex;
use utils::get_input_path;

type OperationHashMap = Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>>;

#[derive(Debug, Clone, Copy)]
enum OperationType {
    Provider,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Debug, Clone)]
struct Operation {
    left: Option<u16>,
    left_name: String,
    right: Option<u16>,
    right_name: String,
    to: String,
    operation: OperationType,
    result: Option<u16>,
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value {
            "AND" => OperationType::And,
            "OR" => OperationType::Or,
            "LSHIFT" => OperationType::LShift,
            "RSHIFT" => OperationType::RShift,
            _ => panic!("Should not be here"),
        }
    }
}

impl Operation {
    fn new(line: &str) -> Self {
        let re = Regex::new(r#"^(\w+) -> (\w+)"#).unwrap();
        if let Some(captures) = re.captures(line) {
            let left = captures[1].parse().ok();
            let left_name = if left.is_none() {
                captures[1].to_string()
            } else {
                String::from("")
            };
            let right = None;
            let right_name = String::from("");
            let to = captures[2].to_string();
            let operation = OperationType::Provider;
            let result = None;

            return Self {
                left,
                left_name,
                right,
                right_name,
                to,
                operation,
                result,
            };
        }

        let re = Regex::new(r#"(\w+) (\w+) (\w+) -> (\w+)"#).unwrap();
        if let Some(captures) = re.captures(line) {
            let left = captures[1].parse().ok();
            let left_name = if left.is_none() {
                captures[1].to_string()
            } else {
                String::from("")
            };
            let right = captures[3].parse().ok();
            let right_name = if right.is_none() {
                captures[3].to_string()
            } else {
                String::from("")
            };
            let to = captures[4].to_string();
            let operation = captures[2].into();
            let result = None;

            return Self {
                left,
                left_name,
                right,
                right_name,
                to,
                operation,
                result,
            };
        }

        let re = Regex::new(r#"NOT (\w+) -> (\w+)"#).unwrap();
        if let Some(captures) = re.captures(line) {
            let left = captures[1].parse().ok();
            let left_name = if left.is_none() {
                captures[1].to_string()
            } else {
                String::from("")
            };
            let right = None;
            let right_name = String::from("");
            let to = captures[2].to_string();
            let operation = OperationType::Not;
            let result = None;

            return Self {
                left,
                left_name,
                right,
                right_name,
                to,
                operation,
                result,
            };
        }

        panic!("Should not be here!")
    }

    fn get_result(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }
        self.result = match self.operation {
            OperationType::Provider => self.get_result_from_provider(operation_map),
            OperationType::And => self.get_result_from_and(operation_map),
            OperationType::Or => self.get_result_from_or(operation_map),
            OperationType::LShift => self.get_result_from_lshift(operation_map),
            OperationType::RShift => self.get_result_from_rshift(operation_map),
            OperationType::Not => self.get_result_from_not(operation_map),
        }
        .into();

        self.result.unwrap()
    }

    fn get_result_from_provider(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = self.left;

        self.result.unwrap()
    }

    fn get_result_from_not(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = Some(!self.left.unwrap());

        self.result.unwrap()
    }

    fn get_result_from_or(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map.clone())
                .into();
        }

        if self.right.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.right_name)
                .unwrap()
                .clone();
            self.right = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = Some(self.left.unwrap() | self.right.unwrap());

        self.result.unwrap()
    }

    fn get_result_from_and(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map.clone())
                .into();
        }

        if self.right.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.right_name)
                .unwrap()
                .clone();
            self.right = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = Some(self.left.unwrap() & self.right.unwrap());

        self.result.unwrap()
    }

    fn get_result_from_lshift(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map.clone())
                .into();
        }

        if self.right.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.right_name)
                .unwrap()
                .clone();
            self.right = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = Some(self.left.unwrap() << self.right.unwrap());

        self.result.unwrap()
    }

    fn get_result_from_rshift(&mut self, operation_map: OperationHashMap) -> u16 {
        if self.result.is_some() {
            return self.result.unwrap();
        }

        if self.left.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.left_name)
                .unwrap()
                .clone();
            self.left = next_operation
                .borrow_mut()
                .get_result(operation_map.clone())
                .into();
        }

        if self.right.is_none() {
            let next_operation = operation_map
                .borrow_mut()
                .get(&self.right_name)
                .unwrap()
                .clone();
            self.right = next_operation
                .borrow_mut()
                .get_result(operation_map)
                .into();
        }

        self.result = Some(self.left.unwrap() >> self.right.unwrap());

        self.result.unwrap()
    }
}

fn run(input_file: &str) {
    // Preamble
    let operation_map: OperationHashMap = Rc::new(RefCell::new(HashMap::new()));

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let operation = Operation::new(&line);
        let key = operation.to.clone();
        let rrco = Rc::new(RefCell::new(operation));
        operation_map.borrow_mut().insert(key, rrco);
    }
    // Solve
    let a = operation_map.borrow_mut().get("a").unwrap().clone();

    let result = a.borrow_mut().get_result(operation_map);

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let operation_map: OperationHashMap = Rc::new(RefCell::new(HashMap::new()));

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let operation = Operation::new(&line);
        let key = operation.to.clone();
        let rrco = Rc::new(RefCell::new(operation));
        operation_map.borrow_mut().insert(key, rrco);
    }
    // Solve
    let a = operation_map.borrow_mut().get("a").unwrap().clone();

    let result = a.borrow_mut().get_result(operation_map.clone());

    for (_, item) in operation_map.borrow_mut().iter_mut() {
        let mut item = item.borrow_mut();
        item.result = None;
        if !item.left_name.is_empty() {
            item.left = None;
        }
        if !item.right_name.is_empty() {
            item.right = None;
        }
    }
    {
        let b = operation_map.borrow_mut().get("b").unwrap().clone();
        b.borrow_mut().left = Some(result);
    }


    let result2 = a.borrow_mut().get_result(operation_map);

    // Result
    println!("Result is {}", result2);
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
