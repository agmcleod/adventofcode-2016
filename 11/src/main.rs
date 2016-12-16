extern crate read_input;
extern crate init_with;
use init_with::InitWith;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ComponentType {
    Chip, Generator
}

#[derive(Copy, Clone, Debug)]
struct Component<'a> {
    name: &'a str,
    component_type: ComponentType,
}

impl<'a> PartialEq for Component<'a> {
    fn eq(&self, other: &Component) -> bool {
        self.name == other.name && self.component_type == other.component_type
    }
}

struct Elevator<'a> {
    floor: usize,
    slot_one: Option<Component<'a>>,
    slot_two: Option<Component<'a>>,
}

const FLOOR_SPACES: usize = 10;
const FLOORS: usize = 4;

type Floor<'a> = Vec<Component<'a>>;
type NodeState<'a> = [Floor<'a>; FLOORS];

struct Node<'a> {
    connected_nodes: Vec<Node<'a>>,
    node_state: NodeState<'a>,
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        let empty_state: NodeState = {
            let data = Vec::with_capacity(FLOOR_SPACES);
            <[Vec<Component>; FLOORS]>::init_with(|| {
                data.clone()
            })
        };
        Node{ connected_nodes: Vec::new(), node_state: empty_state }
    }

    fn as_string(self: &Node<'a>) -> String {
        let mut result = String::new();
        for floor in self.node_state.iter() {
            for component in floor {
                result.push_str(&component.name[0..2]);
                if component.component_type == ComponentType::Generator {
                    result.push_str("-g");
                }
            }
        }

        result
    }
}

fn build_nodes<'a>(first_state: &NodeState<'a>, elevator: &mut Elevator) -> Node<'a> {
    let mut node_usage: HashMap<&str, bool> = HashMap::new();

    next_node(first_state, elevator, &mut node_usage)
}

fn next_node<'a>(node_state: &NodeState<'a>, elevator: &mut Elevator, node_usage: &mut HashMap<&str, bool>) -> Node<'a> {
    let mut node = Node::new();

    let ref floor = node_state[elevator.floor];
    let move_possibilities = get_safe_things_to_move(&floor);

    let move_possibilities = move_possibilities.iter().filter(|possibility| {
        let floor_without_selected = floor.iter().filter(|&component| !possibility.contains(component)).cloned().collect();
        floor_is_safe(&floor_without_selected, Some(elevator))
    });

    node
}

fn get_safe_things_to_move<'a>(floor: &Floor<'a>) -> Vec<Vec<Component<'a>>> {
    let mut things: Vec<Vec<Component<'a>>> = Vec::with_capacity(FLOOR_SPACES);
    if floor.len() <= 1 {
        return things
    }

    for (i, component) in floor.iter().enumerate() {
        for component_two in &floor[i+1..] {
            if component.component_type != component_two.component_type {
                if component.name == component_two.name {
                    things.push(vec![*component, *component_two]);
                }
            } else {
                things.push(vec![*component, *component_two]);
            }
        }
    }

    things
}

fn floor_is_safe(floor: &Floor, elevator: Option<&Elevator>) -> bool {
    let mut components_to_compare = floor.clone();

    if let Some(e) = elevator {
        if let Some(slot) = e.slot_one {
            components_to_compare.push(slot);
        }
        if let Some(slot) = e.slot_two {
            components_to_compare.push(slot);
        }
    }

    components_to_compare.sort_by(|a, b| a.name.cmp(b.name));
    let generator_count = components_to_compare.iter().fold(0, |sum, component|
        if component.component_type == ComponentType::Generator {
            sum + 1
        } else {
            sum
        }
    );

    let mut it = components_to_compare.iter();
    let mut component_one: Option<Component> = None;

    let mut is_safe = true;
    let mut component_one_set = false;
    loop {
        if component_one_set {
            component_one_set = false;
        } else {
            component_one = match it.next() {
                Some(c) => Some(*c),
                None => break,
            };
        }

        let component_two = match it.next() {
            Some(c) => *c,
            None => {
                if let Some(c1) = component_one {
                    if c1.component_type == ComponentType::Chip && generator_count > 0 {
                        is_safe = false;
                    }
                }
                break
            },
        };

        if let Some(c1) = component_one {
            if c1.name != component_two.name {
                if c1.component_type == ComponentType::Chip && generator_count > 0 {
                    is_safe = false;
                    break
                } else {
                    component_one = Some(component_two);
                    component_one_set = true;
                }
            }
        }
    }

    is_safe
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let text = text.replace(",", "");
    let text = text.replace(".", "");

    let mut first_state: NodeState = {
        let data = Vec::with_capacity(FLOOR_SPACES);
        <[Vec<Component>; FLOORS]>::init_with(|| {
            data.clone()
        })
    };

    let mut column_index = 0;
    for (floor_index, line) in text.lines().enumerate() {
        let mut words = line.split(" ").skip_while(|&w| w != "a" && w != "nothing");
        if let Some(word) = words.next() {
            if word == "nothing" {
                continue
            }
        } else {
            continue
        }

        while let Some(word) = words.next() {
            if word == "generator" || word == "a" || word == "and" || word == "microchip" {
                continue
            }
            let name;
            let component_type = if word.contains("-") {
                name = word.split("-").next().unwrap();
                ComponentType::Chip
            } else {
                name = word;
                ComponentType::Generator
            };

            first_state[floor_index][column_index] = Component{ name: name, component_type: component_type };

            column_index += 1;
        }
    }

    let mut elevator = Elevator{ floor: 0, slot_one: None, slot_two: None };
    let nodes = build_nodes(&first_state, &mut elevator);
}

#[test]
fn test_floor_is_safe() {
    let mut floor: Floor = Vec::with_capacity(FLOOR_SPACES);
    assert_eq!(floor_is_safe(&floor, None), true);

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Chip });
    assert_eq!(floor_is_safe(&floor, None), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor, None), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Elevator{
        floor: 0,
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Generator }),
        slot_two: None,
    };
    let elevator = Some(&elevator);
    assert_eq!(floor_is_safe(&floor, elevator), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Elevator{
        floor: 0,
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Chip }),
        slot_two: None,
    };
    let elevator = Some(&elevator);
    assert_eq!(floor_is_safe(&floor, elevator), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    let elevator = Elevator{
        floor: 0,
        slot_one: Some(Component{ name: "four", component_type: ComponentType::Chip }),
        slot_two: Some(Component{ name: "four", component_type: ComponentType::Generator }),
    };
    let elevator = Some(&elevator);
    assert_eq!(floor_is_safe(&floor, elevator), true);
    floor.clear();
}