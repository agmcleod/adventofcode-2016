use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_text() -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open("input.txt"));
  try!(file.read_to_string(&mut text));
  Ok(text)
}

fn get_code_from_command(start_pos: &[i16; 2], commands: &Vec<&str>, grid: &[[i16; 3]; 3]) -> [i16; 3] {
    let mut result = [0, start_pos[0], start_pos[1]];

    for command in commands {
        if *command == "" {
            continue
        }

        match *command {
            "L" => {
                result[1] -=1;
            },
            "R" => {
                result[1] += 1;
            },
            "U" => {
                result[2] -= 1;
            },
            "D" => {
                result[2] += 1;
            },
            _ => {},
        }

        if result[1] < 0 {
            result[1] = 0;
        }
        if result[2] < 0 {
            result[2] = 0;
        }

        if result[1] > 2 {
            result[1] = 2;
        }
        if result[2] > 2 {
            result[2] = 2;
        }
    }

    result[0] = grid[result[2] as usize][result[1] as usize];

    result
}

fn main() {
    let text = match read_text() {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err),
    };

    let grid = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    let mut start_pos = [1, 1];

    let mut codes = Vec::<String>::new();

    let lines: Vec<&str> = text.split("\n").collect();
    for line in lines {
        if line == "" {
            continue
        }
        let commands: Vec<&str> = line.split("").collect();

        let result = get_code_from_command(&start_pos, &commands, &grid);
        start_pos[0] = result[1];
        start_pos[1] = result[2];
        codes.push(result[0].to_string());
    }

    println!("{}", codes.join(""));
}
