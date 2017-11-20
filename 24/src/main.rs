extern crate read_input;
extern crate astar;
use astar::TileType;

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let mut targets: Vec<(usize, usize, u32)> = Vec::new();
    let mut grid: Vec<Vec<TileType>> = Vec::new();
    let mut start_pos = (0, 0);
    for (r, line) in text.lines().enumerate() {
        let mut row: Vec<TileType> = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                row.push(TileType::Unpassable);
            } else {
                row.push(TileType::Open);
                match ch.to_digit(10) {
                    Some(n) => {
                        targets.push((r, c, n));
                        if n == 0 {
                            start_pos = (r, c);
                        }
                    },
                    None => {},
                }
            }
        }
        grid.push(row);
    }

    println!("{:?}", targets);
    println!("{:?}", start_pos);
}
