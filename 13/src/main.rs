extern crate astar;

use std::collections::{HashSet};

const GRID_ROWS: usize = 45;
const GRID_COLS: usize = 45;

fn is_coordinate_open(x: &usize, y: &usize) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y;
    let n = n + 1362; // puzzle input
    n.count_ones() % 2 == 0
}

fn main() {
    let target = (31, 39);
    let mut tiles: Vec<Vec<astar::TileType>> = Vec::new();

    for y in 0..GRID_ROWS {
        let mut row: Vec<astar::TileType> = Vec::new();
        for x in 0..GRID_COLS {
            if is_coordinate_open(&x, &y) {
                row.push(astar::TileType::Open);
            } else {
                row.push(astar::TileType::Unpassable);
            }
        }
        tiles.push(row);
    }

    let tracked_positions = astar::find_path(&tiles, (1, 1), target);

    println!("{}", tracked_positions.len());

    let mut scanned_locations: HashSet<(usize, usize)> = HashSet::new();
    scanned_locations.insert((1, 1));
    let mut steps = 1;

    let mut locations: Vec<(usize, usize)> = vec![(1, 1)];
    while steps <= 50 {
        let mut temp_locations: Vec<(usize, usize)> = Vec::with_capacity(16); // 16 because potential 4 x 4
        for location in locations {
            let locations = astar::get_neighbours(&location, &tiles);
            for loc in locations {
                if !scanned_locations.contains(&loc) {
                    scanned_locations.insert(loc);
                    temp_locations.push(loc);
                }
            }
        }
        locations = temp_locations;
        steps += 1;
    }

    println!("{}", scanned_locations.len());
}
