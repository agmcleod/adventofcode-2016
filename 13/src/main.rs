use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};

const GRID_ROWS: usize = 45;
const GRID_COLS: usize = 45;

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileType {
    Wall, Open
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Location {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_coordinate_open(x: &usize, y: &usize) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y;
    let n = n + 1362; // puzzle input
    n.count_ones() % 2 == 0
}

fn distance_to_target(location: &(usize, usize), target: &(usize, usize)) -> usize {
    let mut x_diff = location.1 as i16 - target.1 as i16;
    let mut y_diff = location.0 as i16 - target.0 as i16;
    if x_diff < 0 {
        x_diff *= -1;
    }
    if y_diff < 0 {
        y_diff *= -1;
    }

    x_diff as usize + y_diff as usize
}

fn get_neighbours(pos: &(usize, usize), tiles: &[[TileType; GRID_COLS]; GRID_ROWS]) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(4);

    if pos.0 > 0 {
        let tile_type = &tiles[pos.1][pos.0 - 1];
        if *tile_type == TileType::Open {
            neighbours.push((pos.0 - 1, pos.1));
        }
    }

    if pos.0 < GRID_COLS - 1 {
        let tile_type = &tiles[pos.1][pos.0 + 1];
        if *tile_type == TileType::Open {
            neighbours.push((pos.0 + 1, pos.1));
        }
    }

    if pos.1 > 0 {
        let tile_type = &tiles[pos.1 - 1][pos.0];
        if *tile_type == TileType::Open {
            neighbours.push((pos.0, pos.1 - 1));
        }
    }

    if pos.1 < GRID_ROWS - 1 {
        let tile_type = &tiles[pos.1 + 1][pos.0];
        if *tile_type == TileType::Open {
            neighbours.push((pos.0, pos.1 + 1));
        }
    }

    neighbours
}

fn main() {
    let target = (31, 39);
    let mut tiles = [[TileType::Wall; GRID_COLS]; GRID_ROWS];

    for (y, row) in tiles.iter_mut().enumerate() {
        for (x, tile_type) in row.iter_mut().enumerate() {
            if is_coordinate_open(&x, &y) {
                *tile_type = TileType::Open;
            }
        }
    }

    let mut closed: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    costs.insert((1, 1), 0);

    let mut heap = BinaryHeap::new();
    heap.push(Location{ position: (1, 1), cost: 0 });

    let mut tracked_positions: Vec<(usize, usize)> = Vec::new();
    let mut scanned_locations: HashSet<(usize, usize)> = HashSet::new();
    scanned_locations.insert((1, 1));

    while let Some(location) = heap.pop() {
        if location.position.0 == target.0 && location.position.1 == target.1 {
            let mut pos = closed.get(&location.position).unwrap();
            tracked_positions.push(location.position);
            loop {
                if let Some(p) = closed.get(&pos) {
                    tracked_positions.push(*p);
                    pos = p;
                } else {
                    break
                }
            }
            break
        }
        let neighbours = get_neighbours(&location.position, &tiles);
        for neighbour in neighbours {
            let new_cost = costs.get(&location.position).unwrap() + 1;
            if !costs.contains_key(&neighbour) || new_cost < *costs.get(&neighbour).unwrap() {
                heap.push(Location{ position: neighbour, cost: new_cost + distance_to_target(&neighbour, &target) });
                closed.insert(neighbour, location.position);
                costs.insert(neighbour, new_cost);
            }

            if distance_to_target(&neighbour, &target) <= 50 {
                scanned_locations.insert(neighbour);
            }
        }
    }

    for (y, row) in tiles.iter().enumerate() {
        let mut x = 0;
        println!("{}", row.iter().map(|tile_type| {
            let pos = (x, y);
            x += 1;
            if *tile_type == TileType::Wall {
                "#"
            } else if tracked_positions.contains(&pos) {
                "O"
            } else {
                "."
            }
        }).collect::<Vec<&str>>().join(""));
    }

    println!("{}", tracked_positions.len());
    println!("{}", scanned_locations.len());
}
