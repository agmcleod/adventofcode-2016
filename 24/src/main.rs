extern crate read_input;
extern crate astar;
extern crate permutohedron;
use astar::TileType;
use permutohedron::Heap;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let mut targets: Vec<(usize, usize, u32)> = Vec::new();
    let mut grid: Vec<Vec<TileType>> = Vec::new();
    let mut start_pos = (0, 0, 0);
    for (r, line) in text.lines().enumerate() {
        let mut row: Vec<TileType> = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                row.push(TileType::Unpassable);
            } else {
                row.push(TileType::Open);
                match ch.to_digit(10) {
                    Some(n) => {
                        if n == 0 {
                            start_pos = (c, r, n);
                        } else {
                            targets.push((c, r, n));
                        }
                    },
                    None => {},
                }
            }
        }
        grid.push(row);
    }

    let heap = Heap::new(&mut targets);
    let mut permutations = Vec::new();
    for data in heap {
        // always put the zero in front
        let mut permutation = vec![start_pos];
        permutation.append(&mut data.clone());
        // part 2
        permutation.append(&mut vec![start_pos]);
        permutations.push(permutation);
    }

    // key is (from,to). The number itself. Low is first, high is second.
    // value is distance, path
    let mut path_cache: HashMap<(u32, u32), (usize, Vec<(usize, usize)>)> = HashMap::new();

    println!("{:?}", permutations[0]);

    let mut steps = 10000;
    for permutation in permutations.iter() {
        let mut permutation_count = 0;

        for pair in permutation.windows(2) {
            let from = cmp::min(pair[0].2, pair[1].2);
            let to = cmp::max(pair[0].2, pair[1].2);
            if !path_cache.contains_key(&(from, to)) {
                let path = astar::find_path(&grid, (pair[0].0, pair[0].1), (pair[1].0, pair[1].1));
                permutation_count += path.len() - 1;
                path_cache.insert((from, to), (path.len() - 1, path));
            } else {
                permutation_count += path_cache.get(&(from, to)).unwrap().0;
            }
        }
        steps = cmp::min(steps, permutation_count);
    }

    println!("steps: {}", steps);
}
