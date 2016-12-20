extern crate read_input;
extern crate init_with;
use init_with::InitWith;
use std::cmp::Ordering;
use std::collections::HashMap;

const NUM_OF_PAIRS: usize = 7;

#[derive(Clone, Debug, PartialEq)]
enum ComponentType {
    Chip,
    Gen,
}

#[derive(Clone, Debug)]
struct Component {
    name: String,
    c_type: ComponentType,
}

type Floor = Vec<Component>;

#[derive(Debug)]
struct Move {
    floor: usize,
    count: usize,
    floors: Vec<Floor>,
}

impl Move {
    fn new(floor: usize, count: usize, floors: Vec<Floor>) -> Move {
        Move{ floor: floor, count: count, floors: floors }
    }
}

fn adjacents(current: usize, count: usize, floors: Vec<Floor>, seen_states: &mut HashMap<String, bool>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut combinations: Vec<Vec<Component>> = Vec::with_capacity(10);
    let floor = floors.get(current).unwrap();

    for (i, component) in floor.iter().enumerate() {
        combinations.push(vec![component.clone()]);
        for component_two in &floor[i+1..] {
            combinations.push(vec![component.clone(), component_two.clone()]);
        }
    }

    let mut possible_floors: Vec<usize> = Vec::with_capacity(2);

    if current == 0 {
        possible_floors.push(1);
    } else if current == 3 {
        possible_floors.push(2);
    } else {
        possible_floors.push(current - 1);
        possible_floors.push(current + 1);
    }

    let deltas: [i16; 2] = [-1, 1];
    for delta in deltas.iter() {
        combinations.sort_by(|a, b| {
            let mut a_len: i16 = a.len() as i16;
            let mut b_len: i16 = b.len() as i16;
            a_len *= -delta;
            b_len *= -delta;
            a_len.cmp(&b_len)
        });

        let new_floor = current as i16 + delta;
        if new_floor < 0 || new_floor > 3 {
            continue
        }

        let new_floor: usize = new_floor as usize;

        for combo in combinations.iter_mut() {
            let mut next_floors = floors.clone();
            next_floors[current] = floors.get(current).unwrap().iter().filter(|c|
                combo.iter().fold(true, |acc, combo_component|
                    if acc && combo_component.name == c.name && combo_component.c_type == c.c_type {
                        false
                    } else {
                        acc
                    }
                )
            ).cloned().collect();

            for combo_component in combo {
                next_floors[new_floor].push(combo_component.clone());
            }

            if state_is_invalid(&next_floors) {
                continue
            }

            let groups = format!("{}{}", new_floor, get_groups(&next_floors));
            if !seen_states.contains_key(&groups) {
                seen_states.insert(groups, true);
                let next_move = Move::new(new_floor, count+1, next_floors);
                moves.push(next_move);
            }
        }
    }
    moves
}

fn get_groups(floors: &Vec<Floor>) -> String {
    let mut results = {
        let pair = Vec::with_capacity(2);
        <[Vec<usize>; NUM_OF_PAIRS]>::init_with(|| {
            pair.clone()
        })
    };
    let mut names: Vec<String> = Vec::with_capacity(NUM_OF_PAIRS);

    for floor in floors {
        for c in floor {
            if !names.contains(&c.name) {
                names.push(c.name.clone());
            }
        }
    }

    let mut index = 0;
    for name in names {
        let mut pair: Vec<usize> = Vec::with_capacity(2);
        for (i, floor) in floors.iter().enumerate() {
            if floor.iter().filter(|c| c.name == name && c.c_type == ComponentType::Gen).count() > 0 {
                pair.push(i);
            }
            if floor.iter().filter(|c| c.name == name && c.c_type == ComponentType::Chip).count() > 0 {
                pair.push(i);
            }
        }
        results[index] = pair;
        index += 1;
    }

    results.sort_by(|a, b|
        if a.len() == 0 {
            Ordering::Less
        } else if b.len() == 0 {
            Ordering::Greater
        } else {
            a[0].cmp(&b[0])
        }
    );

    results.iter().map(|pair| {
        pair.iter().map(|&v| v.to_string()).collect::<Vec<String>>().join("")
    }).collect::<Vec<String>>().join("")
}

fn search(floors: Vec<Floor>, goal: usize, seen_states: &mut HashMap<String, bool>) -> usize {
    let mut next_moves: Vec<Move> = Vec::with_capacity(5);
    let groups = get_groups(&floors);
    seen_states.insert(format!("{}{}", 0, groups), true);
    let moves = adjacents(0, 0, floors, seen_states);
    for m in moves {
        next_moves.push(m);
    }

    let mut move_count = 0;
    'outer: loop {
        let mut more_moves: Option<Vec<Move>> = None;
        for m in next_moves.iter_mut() {
            if m.count > move_count {
                move_count = m.count;
            }

            if m.floors.get(3).unwrap().len() == goal {
                break 'outer;
            }
            let adjacent_results = adjacents(m.floor, m.count, m.floors.clone(), seen_states);
            more_moves = match more_moves {
                Some(mut mm) => {
                    for r in adjacent_results {
                        mm.push(r);
                    }
                    Some(mm)
                },
                None => {
                    Some(adjacent_results)
                }
            };
        }

        next_moves = match more_moves {
            Some(more_moves) => more_moves,
            None => break,
        }
    }

    move_count
}

fn state_is_invalid(floors: &Vec<Floor>) -> bool {
    for floor in floors {
        let chips: Vec<&Component> = floor.iter().filter(|c| c.c_type == ComponentType::Chip).collect();
        let gens: Vec<&Component> = floor.iter().filter(|c| c.c_type == ComponentType::Gen).collect();
        for chip in chips {
            if gens.len() > 0 && gens.iter().filter(|c| c.c_type == chip.c_type && c.name == chip.name).count() > 0 {
                return true
            }
        }
    }

    false
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let mut count = 0;

    let mut floors: Vec<Floor> = text.lines().map(|line| {
        let stuff: Vec<&str> = line.split("a ").map(|bit| bit.trim()).collect();
        let generators: Vec<&str> = stuff.iter().filter(|&s| s.contains("generator")).cloned().collect();
        let chips: Vec<&str> = stuff.iter().filter(|&s| s.contains("microchip")).cloned().collect();

        let mut floor: Floor = Vec::with_capacity(stuff.len());

        for gen in generators {
            floor.push(Component{ name: String::from(gen.split(" ").next().unwrap()), c_type: ComponentType::Gen });
            count += 1;
        }

        for chip in chips {
            floor.push(Component{ name: String::from(chip.split("-").next().unwrap()), c_type: ComponentType::Chip });
            count += 1;
        }

        floor
    }).collect();

    let mut seen_states: HashMap<String, bool> = HashMap::new();
    println!("Moves {} to obtain goal: {}", search(floors.clone(), count, &mut seen_states), count);

    let mut seen_states: HashMap<String, bool> = HashMap::new();
    floors[0].push(Component{ name: String::from("elerium"), c_type: ComponentType::Gen });
    floors[0].push(Component{ name: String::from("elerium"), c_type: ComponentType::Chip });
    floors[0].push(Component{ name: String::from("dilithium"), c_type: ComponentType::Gen });
    floors[0].push(Component{ name: String::from("dilithium"), c_type: ComponentType::Chip });
    println!("Moves {} to obtain goal: {}", search(floors, count + 4, &mut seen_states), count);
}