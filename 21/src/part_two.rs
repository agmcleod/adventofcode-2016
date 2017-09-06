extern crate read_input;
use std::collections::HashMap;

use std;
use utils::*;

trait Command {
    fn apply(&self, &mut [u8]);
}

struct SwapPosition {
    from: usize,
    to: usize,
}

impl Command for SwapPosition {
    fn apply(&self, data: &mut [u8]) {
        data.swap(self.from, self.to);
    }
}

struct SwapLetter {
    target: u8,
    replacement: u8,
}

impl Command for SwapLetter {
    fn apply(&self, data: &mut [u8]) {
        let clone = data.to_vec();
        for (i, v) in clone.iter().enumerate() {
            if *v == self.target {
                data[i] = self.replacement;
            } else if *v == self.replacement {
                data[i] = self.target;
            }
        }
    }
}

struct MovePosition {
    from: usize,
    to: usize,
}

impl Command for MovePosition {
    fn apply(&self, data: &mut [u8]) {
         let mut data_as_vec: Vec<u8> = data.iter().map(|n| *n).collect();
         let from_v = data_as_vec.remove(self.from);
         data_as_vec.insert(self.to, from_v);
         let new_data = data_as_vec.as_slice();
         data.copy_from_slice(new_data);
    }
}

struct RotateFromLetter {
    letter: u8,
}

impl Command for RotateFromLetter {
    fn apply(&self, data: &mut [u8]) {
        let mut data_v = data.to_vec();
        let mut index = match data_v.iter().position(|n| *n == self.letter) {
            Some(index) => index,
            None => panic!("Letter not found for rotate: {:?} in {:?}", std::str::from_utf8(&[self.letter]), std::str::from_utf8(data)),
        };

        if index != 0 && index % 2 == 0 {
            index += data_v.len();
        }
        let count = (index / 2 + 1) % data_v.len();

        for _ in 0..count {
            rotate_vec_left(&mut data_v);
        }

        data.copy_from_slice(data_v.as_slice());
    }
}

struct Rotate {
    direction: String,
    steps: usize,
}

impl Command for Rotate {
    fn apply(&self, data: &mut [u8]) {
        let mut data_v = data.to_vec();
        for _ in 0..self.steps {
            if self.direction == "left".to_string() {
                rotate_vec_left(&mut data_v);
            } else {
                rotate_vec_right(&mut data_v);
            }
        }

        data.copy_from_slice(data_v.as_slice());
    }
}

struct ReverseSection {
    start: usize,
    end: usize,
}

impl Command for ReverseSection {
    fn apply(&self, data: &mut [u8]) {
        let mut data_v = data.to_vec();
        let clone = data_v.clone();

        for i in 0..(self.end - self.start + 1) {
            data_v[i + self.start] = clone[self.end - i];
        }

        data.copy_from_slice(data_v.as_slice());
    }
}

fn get_command(line: &str) -> Option<Box<Command>> {
    if line.starts_with("swap position") {
        let positions = get_two_numbers(line);

        return Some(Box::new(
            SwapPosition{
                from: positions[1],
                to: positions[0],
            }
        ))
    }

    if line.starts_with("swap letter") {
        let line = line.replace("swap letter ", "").replace("with letter", "");
        let letters: Vec<u8> = line.split_whitespace().map(|v| v.as_bytes()[0]).collect();
        return Some(Box::new(
            SwapLetter{
                target: letters[1],
                replacement: letters[0],
            }
        ))
    }

    if line.starts_with("move position") {
        let positions = get_two_numbers(line);

        return Some(Box::new(
            MovePosition{
                from: positions[1],
                to: positions[0],
            }
        ))
    }

    if line.starts_with("rotate based") {
        let letter = line.split_whitespace().last().unwrap();

        return Some(Box::new(
            RotateFromLetter{
                letter: letter.as_bytes()[0]
            }
        ))
    }

    if line.starts_with("rotate") {
        let direction = line.split_whitespace().nth(1).unwrap();
        let steps = match line.split_whitespace().nth(2).unwrap().parse::<usize>() {
            Ok(n) => n,
            Err(_) => panic!("Could not parse num from: {}", line),
        };

        let direction = if direction == "left" {
            "right"
        } else {
            "left"
        };

        return Some(Box::new(
            Rotate{
                direction: direction.to_string(),
                steps: steps,
            }
        ))
    }

    if line.starts_with("reverse") {
        let numbers = get_two_numbers(line);

        return Some(Box::new(
            ReverseSection{
                start: numbers[0],
                end: numbers[1],
            }
        ))
    }

    None
}

pub fn run_p2(input: &String) {
    let str_bytes = "fbgdceah".as_bytes();
    let mut bytes: &mut[u8] = &mut [0u8; 8];
    bytes.clone_from_slice(str_bytes);

    for cmd in input.lines().rev() {
        if let Some(command) = get_command(cmd) {
            let before = String::from_utf8(bytes.iter().cloned().collect::<Vec<u8>>()).unwrap();
            command.apply(&mut bytes);
            println!("{} --- {} --- {}", before, cmd, String::from_utf8(bytes.iter().cloned().collect::<Vec<u8>>()).unwrap());
        } else {
            panic!("Could not find: {}", cmd);
        }
    }

    println!("{:?}", std::str::from_utf8(bytes));
}
