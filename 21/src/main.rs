extern crate read_input;
use read_input::read_text;

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
        let index = match data_v.iter().position(|n| *n == self.letter) {
            Some(index) => index,
            None => panic!("Letter not found for rotate: {:?} in {:?}", std::str::from_utf8(&[self.letter]), std::str::from_utf8(data)),
        };
        let mut count = 1 + index;
        if index >= 4 {
            count += 1;
        }

        for _ in 0..count {
            rotate_vec_right(&mut data_v);
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
            if self.direction == "right".to_string() {
                rotate_vec_right(&mut data_v);
            } else {
                rotate_vec_left(&mut data_v);
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

fn rotate_vec_right(data: &mut Vec<u8>) {
    let copy = data.clone();
    for (i, v) in copy.iter().enumerate() {
        let index = if i == data.len() - 1 {
            0
        } else {
            i + 1
        };
        data[index] = *v;
    }
}

fn rotate_vec_left(data: &mut Vec<u8>) {
    let copy = data.clone();
    for (i, v) in copy.iter().enumerate() {
        let index = if i == 0 {
            data.len() - 1
        } else {
            i - 1
        };
        data[index] = *v;
    }
}

fn get_two_numbers(line: &str) -> Vec<usize> {
    line.split_whitespace().filter(|&n| {
        match n.parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }).map(|num| num.parse::<usize>().unwrap()).collect()
}

fn get_command(line: &str) -> Option<Box<Command>> {
    if line.starts_with("swap position") {
        let positions = get_two_numbers(line);

        return Some(Box::new(
            SwapPosition{
                from: positions[0],
                to: positions[1],
            }
        ))
    }

    if line.starts_with("swap letter") {
        let line = line.replace("swap letter ", "").replace("with letter", "");
        let letters: Vec<u8> = line.split_whitespace().map(|v| v.as_bytes()[0] ).collect();
        return Some(Box::new(
            SwapLetter{
                target: letters[0],
                replacement: letters[1],
            }
        ))
    }

    if line.starts_with("move position") {
        let positions = get_two_numbers(line);

        return Some(Box::new(
            MovePosition{
                from: positions[0],
                to: positions[1],
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

fn main() {
    let str_bytes = "abcdefgh".as_bytes();
    let mut bytes: &mut[u8] = &mut [0u8; 8];
    bytes.clone_from_slice(str_bytes);
    let input = match read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    for cmd in input.lines() {
        if let Some(command) = get_command(cmd) {
            command.apply(&mut bytes);
            println!("{} => {:?}", cmd, std::str::from_utf8(bytes));
        } else {
            panic!("Could not find: {}", cmd);
        }
    }
}
