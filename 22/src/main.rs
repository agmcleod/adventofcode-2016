extern crate regex;
extern crate read_input;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Node {
    coords: String,
    size: usize,
    used: usize,
    avail: usize,
    coords_as_num: Option<[usize; 2]>,
}

impl Node {
    fn new(coords: String, size: String, used: String, avail: String) -> Node {
        Node{
            coords: coords,
            size: size.replace("T", "").parse().ok().expect("Failed to parse size"),
            used: used.replace("T", "").parse().ok().expect("Failed to parse used"),
            avail: avail.replace("T", "").parse().ok().expect("Failed to parse avail"),
            coords_as_num: None,
        }
    }

    fn get_coords(&mut self) -> [usize; 2] {
        if self.coords_as_num == None {
            let mut coords = [0, 0];
            for (i, n) in self.coords.replace("x", "").split("y").enumerate() {
                coords[i] = n.parse().ok().expect("Could not parse n");
            }
            self.coords_as_num = Some(coords);
        }
        self.coords_as_num.unwrap()
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

    let mut pairs: Vec<[String; 2]> = Vec::new();

    for (key, node) in &nodes {
        for (key2, node2) in &nodes {
            if key != key2 {
                let pair = [node, node2];

                if pair[0].used > 0 && pair[0].used <= pair[1].avail {
                    let pair = [pair[0].coords.clone(), pair[1].coords.clone()];

                    pairs.push(pair);
                }
            }
        }
    }

    println!("{:?}", pairs.len());
}
