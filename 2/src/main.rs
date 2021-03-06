extern crate read_input;

fn get_code_from_command(start_pos: &[i16; 2], commands: &Vec<&str>, grid: &Vec<Vec<&str>>) -> ([i16; 2], String) {
    let mut result = [start_pos[0], start_pos[1]];

    let max = grid.len() as i16;

    for command in commands {
        if *command == "" {
            continue
        }

        let current_result = [result[0], result[1]];

        match *command {
            "L" => {
                result[0] -=1;
            },
            "R" => {
                result[0] += 1;
            },
            "U" => {
                result[1] -= 1;
            },
            "D" => {
                result[1] += 1;
            },
            _ => {},
        }

        if result[0] < 0 || result[0] >= max {
            result[0] = current_result[0];
        } else if result[1] < 0 || result[1] >= max {
            result[1] = current_result[1];
        } else if grid[result[1] as usize][result[0] as usize] == "" {
            result[0] = current_result[0];
            result[1] = current_result[1];
        }
    }

    return (result, String::from(grid[result[1] as usize][result[0] as usize]))
}

fn solve_for_grid(start_pos: &mut [i16; 2], grid: &Vec<Vec<&str>>, commands: &str) {
    let mut codes = Vec::<String>::new();

    for line in commands.lines() {
        let commands: Vec<&str> = line.split("").collect();

        let (result, code) = get_code_from_command(&start_pos, &commands, &grid);
        start_pos[0] = result[0];
        start_pos[1] = result[1];
        codes.push(code);
    }

    println!("{}", codes.join(""));
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err),
    };

    let grid = vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"]
    ];

    let gridp2 = vec![
        vec!["", "", "1", "", ""],
        vec!["", "2", "3", "4", ""],
        vec!["5", "6", "7", "8", "9"],
        vec!["", "A", "B", "C", ""],
        vec!["", "", "D", "", ""],
    ];

    let mut start_pos = [1, 1];

    solve_for_grid(&mut start_pos, &grid, &text);
    start_pos = [0, 2];
    solve_for_grid(&mut start_pos, &gridp2, &text);
}
