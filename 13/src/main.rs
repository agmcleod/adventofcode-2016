use std::cmp;

const GRID_ROWS: usize = 50;
const GRID_COLS: usize = 40;

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileType {
    Wall, Open, Used
}

#[derive(Debug)]
struct Tile {
    cost_from_origin: i16,
    cost_to_target: i16,
    score: i16,
    tile_type: TileType,
}

impl Copy for Tile {}
impl Clone for Tile {
    fn clone(&self) -> Tile {
        *self
    }
}

fn is_coordinate_open(x: &usize, y: &usize) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y;
    let n = n + 1362; // puzzle input
    n.count_ones() % 2 == 0
}

fn distance_to_target(x: usize, y: usize, target: &(usize, usize)) -> i16 {
    let mut x_diff = x as i16 - target.0 as i16;
    let mut y_diff = y as i16 - target.1 as i16;
    if x_diff < 0 {
        x_diff *= -1;
    }
    if y_diff < 0 {
        y_diff *= -1;
    }

    x_diff + y_diff
}

fn get_next_position(pos: &(usize, usize), tiles: &[[Tile; GRID_COLS]; GRID_ROWS]) -> (usize, usize) {
    let mut score = 10000;

    let mut next_pos = (pos.0, pos.1);

    if pos.0 > 0 {
        let tile = &tiles[pos.1][pos.0 - 1];
        if tile.tile_type == TileType::Open {
            println!("{:?}", tile);
            score = tile.score;
            let (ref mut x, _) = next_pos;
            *x = pos.0 - 1;
        }
    }

    if pos.0 < GRID_COLS - 1 {
        let tile = &tiles[pos.1][pos.0 + 1];

        if tile.tile_type == TileType::Open && tile.score == cmp::min(tile.score, score) {
            println!("{:?}", tile);
            score = tile.score;
            let (ref mut x, _) = next_pos;
            *x = pos.0 + 1;
        }
    }

    if pos.1 > 0 {
        let tile = &tiles[pos.1 - 1][pos.0];
        if tile.tile_type == TileType::Open && tile.score == cmp::min(tile.score, score) {
            println!("{:?}", tile);
            score = tile.score;
            let (_, ref mut y) = next_pos;
            *y = pos.1 - 1;
        }
    }

    if pos.1 < GRID_ROWS - 1 {
        let tile = &tiles[pos.1 + 1][pos.0];
        if tile.tile_type == TileType::Open && tile.score == cmp::min(tile.score, score) {
            println!("{:?}", tile);
            let (_, ref mut y) = next_pos;
            *y = pos.1 + 1;
        }
    }

    if next_pos.0 == pos.0 && next_pos.1 == pos.1 {
        panic!("Position was unchanged for: {}, {}", next_pos.0, next_pos.1);
    }

    next_pos
}

fn main() {
    let target = (31, 39);
    let mut pos = (0, 0);
    let mut tiles = [[Tile{ cost_from_origin: 0, cost_to_target: 0, score: 0, tile_type: TileType::Wall }; GRID_COLS]; GRID_ROWS];

    for (y, row) in tiles.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if is_coordinate_open(&x, &y) {
                if x == 0 && y == 0 {
                    tile.tile_type = TileType::Used;
                } else {
                    tile.tile_type = TileType::Open;
                }

                tile.cost_to_target = distance_to_target(x, y, &target);
                tile.cost_from_origin = distance_to_target(x, y, &pos);
                tile.score = tile.cost_to_target + tile.cost_from_origin;
            } else {
                tile.tile_type = TileType::Wall;
            }
        }
    }

    for row in tiles.iter() {
        println!("{}", row.iter().map(|tile|
            if tile.tile_type == TileType::Wall {
                "#"
            } else {
                "."
            }
        ).collect::<Vec<&str>>().join(""));
    }

    let mut moves = 0;

    loop {
        let next_pos = get_next_position(&pos, &tiles);
        pos = (next_pos.0, next_pos.1);
        let ref mut tile = tiles[pos.1][pos.0];
        tile.tile_type = TileType::Used;
        println!("{}, {} {:?}", pos.0, pos.1, tile.tile_type);
        moves += 1;
        if pos.0 == target.0 && pos.1 == target.1 {
            break
        }
    }

    println!("{}", moves);
}
