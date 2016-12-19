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
    id: usize,
    name: &'a str,
    component_type: ComponentType,
}

impl<'a> PartialEq for Component<'a> {
    fn eq(&self, other: &Component) -> bool {
        self.name == other.name && self.component_type == other.component_type
    }
}

const FLOOR_SPACES: usize = 10;
const FLOORS: usize = 4;

type Floor<'a> = Vec<Component<'a>>;

type NodeState<'a> = [Floor<'a>; FLOORS];

struct Node<'a> {
    connected_nodes: Vec<Node<'a>>,
    node_state: NodeState<'a>,
    elevator: usize,
}

impl<'a> Node<'a> {
    fn new(node_state: NodeState<'a>, elevator: usize) -> Node<'a> {
        let mut node = Node{ connected_nodes: Vec::new(), node_state: node_state, elevator: elevator };
        for floor in node.node_state.iter_mut() {
            floor.sort_by(|a, b| a.name.cmp(b.name));
        }
        node
    }
}

fn as_string<'a>(node_state: &NodeState<'a>) -> String {
    node_state.iter().map(|floor| {
        floor.iter().map(|component| {
            component.id.to_string()
        }).collect::<Vec<String>>().join("")
    }).collect::<Vec<String>>().join("")
}

fn build_nodes<'a>(first_state: NodeState<'a>) -> Node<'a> {
    let mut node_usage: HashMap<String, bool> = HashMap::new();
    let mut node = Node::new(first_state, 0);
    let mut working_set: Vec<&mut Node<'a>> = vec![&mut node];
    loop {
        let mut next_set: Vec<&mut Node<'a>> = Vec::with_capacity(10);
        for node in working_set {
            let move_possibilities;
            {
                move_possibilities = get_safe_things_to_move(&node.node_state[node.elevator]);
            }

            if node.node_state[3].len() == FLOOR_SPACES {
                continue
            }

            let mut possible_floors: Vec<usize> = Vec::with_capacity(2);

            if node.elevator == 0 {
                possible_floors.push(1);
            } else if node.elevator == 3 {
                possible_floors.push(2);
            } else {
                possible_floors.push(node.elevator - 1);
                possible_floors.push(node.elevator + 1);
            }

            let mut move_possibilities: Vec<Vec<Component<'a>>> = move_possibilities.iter().filter(|possibility| {
                let floor_without_selected = node.node_state[node.elevator].iter().filter(|&component| !possibility.contains(component)).cloned().collect();
                floor_is_safe(&floor_without_selected)
            }).cloned().collect();

            let mut connected_nodes: Vec<Node<'a>> = Vec::new();

            for floor_index in possible_floors {
                let ref floor = node.node_state[floor_index];
                if floor.len() == FLOOR_SPACES {
                    continue
                }
                for possibility in move_possibilities.iter_mut() {
                    for component in possibility.iter_mut() {
                        let mut floor = floor.clone();
                        floor.push(*component);
                        let mut node_state = build_node_state(&node.node_state, floor, floor_index);
                        let to_remove_index = node_state[node.elevator].iter().position(|&c| c.name == component.name).unwrap();
                        node_state[node.elevator].remove(to_remove_index);
                        let node_string = as_string(&node_state);
                        if !node_usage.contains_key(&node_string) {
                            let node = Node::new(node_state, floor_index);
                            connected_nodes.push(node);
                            node_usage.insert(node_string, true);
                        }
                    }

                    if possibility.len() > 1 && floor.len() <= FLOOR_SPACES - possibility.len() {
                        let mut floor = floor.clone();
                        for component in possibility.iter_mut() {
                            floor.push(*component);
                        }

                        let mut node_state = build_node_state(&node.node_state, floor, floor_index);

                        for component in possibility.iter_mut() {
                            let to_remove_index = node_state[node.elevator].iter().position(|&c| c.name == component.name).unwrap();
                            node_state[node.elevator].remove(to_remove_index);
                        }

                        let node_string = as_string(&node_state);
                        if !node_usage.contains_key(&node_string) {
                            let node = Node::new(node_state, floor_index);
                            connected_nodes.push(node);
                            node_usage.insert(node_string, true);
                        }
                    }
                }
            }

            for sub_node in connected_nodes {
                node.connected_nodes.push(sub_node);
                next_set.push(&mut sub_node);
            }
        }

        working_set = next_set;
    }

    node
}

fn build_node_state<'a>(existing_node_state: &NodeState<'a>, modified_floor: Floor<'a>, floor_index: usize) -> NodeState<'a> {
    let mut new_node_state: NodeState<'a> = [
        existing_node_state[0].clone(),
        existing_node_state[1].clone(),
        existing_node_state[2].clone(),
        existing_node_state[3].clone()
    ];
    new_node_state[floor_index] = modified_floor;

    new_node_state
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

fn floor_is_safe(floor: &Floor) -> bool {
    let generator_count = floor.iter().fold(0, |sum, component|
        if component.component_type == ComponentType::Generator {
            sum + 1
        } else {
            sum
        }
    );

    let mut it = floor.iter();
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

    let mut id = 1;

    for (floor_index, line) in text.lines().enumerate() {
        let mut words = line.split(" ").skip_while(|&w| w != "a" && w != "nothing");
        if let Some(word) = words.next() {
            if word == "nothing" {
                continue
            }
        } else {
            continue
        }

        let mut floor: Floor = Vec::new();

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
            floor.push(Component{ id: id, name: name, component_type: component_type });
            id += 1;
        }

        first_state[floor_index] = floor;
    }

    let nodes = build_nodes(first_state);
}

#[test]
fn test_floor_is_safe() {
    let mut floor: Floor = Vec::with_capacity(FLOOR_SPACES);
    assert_eq!(floor_is_safe(&floor), true);

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    assert_eq!(floor_is_safe(&floor), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Chip });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    floor.push(Component{ name: "four", component_type: ComponentType::Generator });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), true);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    floor.push(Component{ name: "four", component_type: ComponentType::Chip });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), false);
    floor.clear();

    floor.push(Component{ name: "one", component_type: ComponentType::Generator });
    floor.push(Component{ name: "one", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Chip });
    floor.push(Component{ name: "two", component_type: ComponentType::Generator });
    floor.push(Component{ name: "three", component_type: ComponentType::Generator });
    floor.push(Component{ name: "four", component_type: ComponentType::Chip });
    floor.push(Component{ name: "four", component_type: ComponentType::Generator });
    floor.sort_by(|a, b| a.name.cmp(b.name));
    assert_eq!(floor_is_safe(&floor), true);
    floor.clear();
}