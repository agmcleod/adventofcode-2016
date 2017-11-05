extern crate regex;
extern crate read_input;

use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Node {
    coords: String,
    size: usize,
    used: usize,
    avail: usize,
    coords_as_num: [usize; 2],
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node{
            coords: self.coords.clone(),
            size: self.size.clone(),
            used: self.used.clone(),
            avail: self.avail.clone(),
            coords_as_num: self.coords_as_num.clone(),
        }
    }
}

const MAX_X: usize = 36;
const MAX_Y: usize = 24;

impl Node {
    fn new(coords: String, size: String, used: String, avail: String) -> Node {
        let mut num_coords = [0, 0];
        for (i, n) in coords.replace("x", "").split("y").enumerate() {
            num_coords[i] = n.parse().ok().expect("Could not parse n");
        }
        Node{
            coords: coords,
            size: size.replace("T", "").parse().ok().expect("Failed to parse size"),
            used: used.replace("T", "").parse().ok().expect("Failed to parse used"),
            avail: avail.replace("T", "").parse().ok().expect("Failed to parse avail"),
            coords_as_num: num_coords,
        }
    }

    fn get_neighbours(&self) -> Vec<String> {
        let mut neighbours: Vec<String> = Vec::new();

        if self.coords_as_num[0] > 0 {
            neighbours.push(format!("x{}y{}", self.coords_as_num[0] - 1, self.coords_as_num[1]));
        }
        if self.coords_as_num[1] > 0 {
            neighbours.push(format!("x{}y{}", self.coords_as_num[0], self.coords_as_num[1] - 1));
        }
        if self.coords_as_num[0] < MAX_X {
            neighbours.push(format!("x{}y{}", self.coords_as_num[0] + 1, self.coords_as_num[1]));
        }
        if self.coords_as_num[1] < MAX_Y {
            neighbours.push(format!("x{}y{}", self.coords_as_num[0], self.coords_as_num[1] + 1));
        }

        neighbours
    }
}

fn find_pairs_for_node(nodes: &HashMap<String, Node>, node: &Node) -> Vec<String> {
    let mut pairs: Vec<String> = Vec::new();

    for (key, node2) in nodes {
        if *key != node.coords {
            if node.used > 0 && node.used <= node2.avail {
                pairs.push(node2.coords.clone());
            }
        }
    }

    pairs
}

// there's only one, this was overkill XD
fn find_first_zero_space_node(nodes: & HashMap<String, Node>, from: &Node) -> Option<Node> {
    let mut scan_list: Vec<&Node> = vec![from];
    let mut used_list: HashSet<String> = HashSet::new();
    loop {
        let mut temp_list: Vec<&Node> = Vec::new();
        let mut any_found = false;
        for node in &scan_list {
            let neighbours = node.get_neighbours();
            for c in neighbours {
                if used_list.contains(&c) {
                    continue
                }
                any_found = true;
                used_list.insert(c.clone());
                let node = nodes.get(&c).unwrap();
                if node.used == 0 {
                    return Some((*node).clone());
                }

                temp_list.push(node);
            }
        }
        scan_list = temp_list.clone();
        if !any_found {
            break
        }
    }

    None
}

fn move_node_data_to_coords(nodes: &HashMap<String, Node>, node: &Node, target: &Node) -> (usize, String, HashMap<String, Node>) {
    let mut scan_list: Vec<(String, HashMap<String, Node>)> = vec![(node.coords.clone(), nodes.clone())];
    let mut used_list: HashSet<String> = HashSet::new();
    let mut count = 1;
    let mut result_state: HashMap<String, Node> = HashMap::new();
    let mut last_move = String::new();
    'main: loop {
        let mut temp_list: Vec<(String, HashMap<String, Node>)> = Vec::new();
        let mut any_found = false;
        for state in &scan_list {
            let node = state.1.get(&state.0).unwrap();
            let neighbours = node.get_neighbours();
            for c in neighbours {
                let mut new_state = state.1.clone();
                let mut new_stuff = {
                    let neighbour = new_state.get(&c).unwrap();
                    let node = new_state.get(&state.0).unwrap();
                    // move on if node already scanned, or if either node can't fit the data
                    if used_list.contains(&c) || neighbour.used > node.size || node.used > neighbour.size {
                        continue
                    }
                    (neighbour.clone(), node.clone())
                };
                let neighbour_used = new_stuff.0.used;
                let neighbour_coords = new_stuff.0.coords.clone();
                new_stuff.0.used = new_stuff.1.used;
                new_stuff.1.used = neighbour_used;
                new_state.insert(new_stuff.0.coords.clone(), new_stuff.0);
                new_state.insert(new_stuff.1.coords.clone(), new_stuff.1);

                if neighbour_coords == target.coords {
                    result_state = new_state;
                    last_move = state.0.clone();
                    break 'main
                }

                temp_list.push((neighbour_coords.clone(), new_state));
                any_found = true;
                used_list.insert(c.clone());
            }
        }
        count += 1;
        scan_list = temp_list.clone();
        if !any_found {
            println!("Ran out of options");
            break
        }
    }

    (count, last_move, result_state)
}

fn get_path_for_data(nodes: &HashMap<String, Node>, data_coords: &String) -> Vec<String> {
    let (mut scan_list, data_used) = {
        let data_node = nodes.get(data_coords).unwrap();
        (vec![(data_node.clone(), vec![data_node.coords.clone()])], data_node.used)
    };
    let mut used_list: HashSet<String> = HashSet::new();
    let target_coords = "x0y0".to_string();

    'main: loop {
        let mut temp_list: Vec<(Node, Vec<String>)> = Vec::new();
        let mut any_found = false;
        for &(ref node, ref path) in &scan_list {
            let neighbours = node.get_neighbours();
            for neighbour_coord in neighbours {
                if neighbour_coord == target_coords {
                    return (*path).clone()
                }
                if used_list.contains(&neighbour_coord) {
                    continue
                }
                used_list.insert(neighbour_coord.clone());
                let neighbour = nodes.get(&neighbour_coord).unwrap();
                if neighbour.size >= data_used {
                    any_found = true;
                    let mut path = (*path).clone();
                    path.push(neighbour.coords.clone());
                    temp_list.push((neighbour.clone(), path));
                }
            }
        }

        scan_list = temp_list;
        if !any_found {
            break
        }
    }

    Vec::new()
}

fn print_nodes(nodes: &HashMap<String, Node>) {
    for y in 0..(MAX_Y+1) {
        let mut row: Vec<String> = Vec::new();
        for x in 0..(MAX_X+1) {
            let node = nodes.get(&format!("x{}y{}", x, y)).unwrap();
            row.push(format!("{}/{}", node.used, node.size));
        }

        println!("{}", row.join("|"));
    }
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let re = Regex::new(r"\s+").unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in text.lines() {
        if line.starts_with("/") {
            let words = re.split(line).collect::<Vec<&str>>();
            let pieces = words[0].split("-").collect::<Vec<&str>>();
            let coords = format!("{}{}", pieces[1], pieces[2]);
            nodes.insert(coords.clone(), Node::new(coords, words[1].to_string(), words[2].to_string(), words[3].to_string()));
        }
    }

    let mut pairs: Vec<String> = Vec::new();

    // part 1
    for (_, node) in &nodes {
        pairs.append(&mut find_pairs_for_node(&nodes, &node));
    }

    println!("part 1 pairs: {:?}", pairs.len());

    if let Some(zero_space) = find_first_zero_space_node(&nodes, nodes.get(&format!("x{}y{}", MAX_X, 0)).unwrap()) {
        println!("\n\nZero space node, from top right {:?}\n\n", zero_space);
        let result = move_node_data_to_coords(&nodes, &zero_space, nodes.get(&format!("x{}y{}", MAX_X, 0)).unwrap());
        println!("Count to move 0 to top right: {}", result.0);
        let data_node = result.2.get(&result.1).unwrap();
        println!("Moved data amount: {} to: {}\n", data_node.used, data_node.coords);


        let path = get_path_for_data(&result.2, &data_node.coords);
        println!("{:?}", path);
    }

}
