#[derive(PartialEq)]
enum Tile {
    Safe,
    Trap,
}

fn get_tile_type(ch: &char) -> Tile {
    if *ch == '^' {
        Tile::Trap
    } else {
        Tile::Safe
    }
}

fn get_tile_type_from_previous(previous: &[Tile; 3]) -> String {
    if (previous[0] == Tile::Trap && previous[1] == Tile::Trap && previous[2] == Tile::Safe) ||
        (previous[0] == Tile::Safe && previous[1] == Tile::Trap && previous[2] == Tile::Trap) ||
        (previous[0] == Tile::Trap && previous[1] == Tile::Safe && previous[2] == Tile::Safe) ||
        (previous[0] == Tile::Safe && previous[1] == Tile::Safe && previous[2] == Tile::Trap)
    {
        return String::from("^")
    }

    String::from(".")
}

fn main() {
    let mut line = String::from(".^^^^^.^^.^^^.^...^..^^.^.^..^^^^^^^^^^..^...^^.^..^^^^..^^^^...^.^.^^^^^^^^....^..^^^^^^.^^^.^^^.^^");
    let mut safe_count = line.replace("^", "").len();
    for _ in 0..399999 {
        let mut new_line = Vec::<String>::with_capacity(line.len());
        for i in 0..line.len() {
            let mut tiles = [Tile::Safe, Tile::Safe, Tile::Safe];
            if i > 0 {
                tiles[0] = get_tile_type(&line.chars().nth(i - 1).unwrap());
            }
            tiles[1] = get_tile_type(&line.chars().nth(i).unwrap());
            if i < line.len() - 1 {
                tiles[2] = get_tile_type(&line.chars().nth(i + 1).unwrap());
            }

            let result = get_tile_type_from_previous(&tiles);
            new_line.push(result);
        }

        line = new_line.join("");

        safe_count += line.replace("^", "").len();
    }

    println!("{}", safe_count);
}
