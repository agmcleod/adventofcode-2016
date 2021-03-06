extern crate read_input;

use std::collections::HashMap;

enum Direction {
    N, E, S, W
}

fn change_direction(direction: Direction, increase: bool) -> Direction {
    match direction {
        Direction::N => {
            if increase {
                return Direction::E;
            } else {
                return Direction::W;
            }
        },
        Direction::E => {
            if increase {
                return Direction::S;
            } else {
                return Direction::N;
            }
        },
        Direction::S => {
            if increase {
                return Direction::W;
            } else {
                return Direction::E;
            }
        },
        Direction::W => {
            if increase {
                return Direction::N;
            } else {
                return Direction::S;
            }
        },
    }
}

fn insert_pos(positions: &mut HashMap<i16, Vec<i16>>, x: i16, y: i16) -> bool {
    if positions.contains_key(&x) {
        let mut ys = positions.get_mut(&x).unwrap();
        if ys.contains(&y) {
            return true
        } else {
            ys.push(y);
            return false
        }
    } else {
        positions.insert(x, vec![y]);
        return false
    }
}

fn print_dupe_coords(x: i16, y: i16) {
    println!("already crossed: {} {} = {}", x.abs(), y.abs(), x.abs() + y.abs());
}

fn main() {
    let mut positions = HashMap::new();
    let mut found_dupe_pos = false;

    match read_input::read_text("input.txt") {
        Ok(text) => {
            let instructions: Vec<&str> = text.split(", ").collect();
            let mut x = 0i16;
            let mut y = 0i16;

            insert_pos(&mut positions, x, y);

            let mut direction = Direction::N;

            for instruction in instructions {
                let instruction = String::from(instruction);
                let bits: Vec<String> = instruction.chars().map(|s| s.to_string()).collect();
                let n: i16 = bits[2..bits.len()].join("").parse().unwrap();

                direction = change_direction(direction, bits[0] == "R");

                match direction {
                    Direction::N => {
                        for i in 1..(n+1) {
                            let contains_already = insert_pos(&mut positions, x, y - i);
                            if contains_already && !found_dupe_pos {
                                print_dupe_coords(x, y - i);
                                found_dupe_pos = true;
                                break
                            }
                        }
                        y -= n;
                    },
                    Direction::E => {
                        for i in 1..(n+1) {
                            let contains_already = insert_pos(&mut positions, x + i, y);
                            if contains_already && !found_dupe_pos {
                                print_dupe_coords(x + i, y);
                                found_dupe_pos = true;
                                break
                            }
                        }
                        x += n;
                    },
                    Direction::S => {
                        for i in 1..(n+1) {
                            let contains_already = insert_pos(&mut positions, x, y + i);
                            if contains_already && !found_dupe_pos {
                                print_dupe_coords(x, y + i);
                                found_dupe_pos = true;
                                break
                            }
                        }
                        y += n;
                    },
                    Direction::W => {
                        for i in 1..(n+1) {
                            let contains_already = insert_pos(&mut positions, x - i, y);
                            if contains_already && !found_dupe_pos {
                                print_dupe_coords(x - i, y);
                                found_dupe_pos = true;
                                break
                            }
                        }
                        x -= n;
                    },
                }
            }

            println!("shortest path: {}", x.abs() + y.abs());
        },
        Err(err) => panic!("{}", err),
    }
}
