extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

const GRID_SIZE: usize = 4;

fn build_options(options_based_on_position: &mut HashMap<(usize, usize), [Option<(usize, usize)>; GRID_SIZE]>) {
    for row in 0i16..4i16 {
        for col in 0i16..4i16 {
            let mut positions: [Option<(usize, usize)>; GRID_SIZE] = [None, None, None, None];
            if row - 1i16 > -1 {
                positions[0] = Some((col as usize, (row - 1i16) as usize));
            }
            if row + 1i16 < GRID_SIZE as i16 {
                positions[1] = Some((col as usize, (row + 1i16) as usize));
            }
            if col - 1i16 > -1 {
                positions[2] = Some(((col - 1i16) as usize, row as usize));
            }
            if col + 1i16 < GRID_SIZE as i16 {
                positions[3] = Some(((col  + 1i16) as usize, row as usize));
            }

            options_based_on_position.insert((col as usize, row as usize), positions);
        }
    }
}

fn door_is_open(character: &char) -> bool {
    *character == 'b'|| *character == 'c' || *character == 'd' || *character == 'e' || *character == 'f'
}

fn get_md5(md5: &mut Md5, v: &String) -> String {
    md5.input(v.as_bytes());
    let result = md5.result_str();
    md5.reset();

    result
}

struct State<'a> {
    path: Vec<&'a str>,
    position: (usize, usize),
    input: String,
}

fn try_position(door_letters: &[&str; 4], options_based_on_position: &HashMap<(usize, usize), [Option<(usize, usize)>; GRID_SIZE]>) {
    let mut md5 = Md5::new();

    let mut options = vec![State{ path: Vec::new(), position: (0, 0), input: String::from("qtetzkpl") }];
    loop {
        let mut new_options: Vec<State> = Vec::with_capacity(options.len() * 4);
        let mut finished = false;
        for state in &options {
            let next_value = get_md5(&mut md5, &state.input);
            let position = &state.position;
            if position.0 == GRID_SIZE - 1 && position.1 == GRID_SIZE - 1 {
                finished = true;
                println!("{} {}", state.path.join(""), state.path.len());
                break
            }
            let positions = options_based_on_position.get(&(position.0, position.1)).unwrap();
            for (i, ch) in next_value[0..GRID_SIZE].chars().enumerate() {
                if door_is_open(&ch) {
                    let position = positions[i];
                    match position {
                        Some(p) => {
                            let mut appended_path = state.path.clone();
                            appended_path.push(door_letters[i]);
                            new_options.push(State{ path: appended_path, position: p, input: format!("{}{}", state.input, door_letters[i]) });
                        },
                        None => continue,
                    }

                }
            }
        }

        if !new_options.len() > 0 {
            options = new_options;
        }

        if finished {
            break
        }
    }
}

fn main() {
    let mut options_based_on_position = HashMap::<(usize, usize), [Option<(usize, usize)>; GRID_SIZE]>::new();

    let door_letters = ["U", "D", "L", "R"];

    build_options(&mut options_based_on_position);

    try_position(&door_letters, &options_based_on_position);
}
