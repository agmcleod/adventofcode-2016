extern crate read_input;

use read_input::read_text;
const COLS: usize = 50;
const ROWS: usize = 6;

type Grid<'a> = [[&'a str; COLS]; ROWS];

fn print_grid<'a>(grid: &'a Grid) {
    println!("");
    for row in grid.iter() {
        println!("{}", row.join(""));
    }
    println!("");
}

struct Command<'a> {
    cmd_name: &'a str,
    args: [&'a str; 4],
}

impl<'a> Command<'a> {
    fn new(text: &str) -> Command {
        let mut name = "";
        let mut args = ["", "", "", ""];
        for (i, piece) in text.split(" ").enumerate() {
            if i == 0 {
                name = piece;
            } else {
                args[i - 1] = piece;
            }
        }

        Command{
            cmd_name: name,
            args: args,
        }
    }

    fn apply(self: &'a Command<'a>, grid: &mut Grid) {
        match self.cmd_name {
            "rect" => {
                let mut dimensions = self.args[0].split("x");
                if let (Some(width), Some(height)) = (dimensions.next(), dimensions.next()) {
                    let height: usize = height.parse().expect("Expected height to be a number");
                    let width: usize = width.parse().expect("Expected length to be a number");
                    for r in 0..height {
                        for c in 0..width {
                            grid[r][c] = "#";
                        }
                    }
                }
            },
            "rotate" => {
                let is_column = self.args[0] == "column";
                let index: usize = self.args[1].split("=").nth(1).unwrap().parse().expect(
                    "index to rotate: did not parse as number"
                );
                let iterations: usize = self.args[3].parse().expect(
                    "iterations did not parse as number"
                );

                if is_column {
                    let mut col_values = ["."; ROWS];
                    for (i, row) in grid.iter().enumerate() {
                        col_values[i] = row.get(index).unwrap();
                    }

                    let start_index = iterations % col_values.len();
                    let mut insert_index = 0;
                    for i in start_index..(col_values.len() + start_index) {
                        grid[i % col_values.len()][index] = col_values.get(insert_index).unwrap();
                        insert_index += 1;
                    }

                } else {
                    let mut new_row = ["."; COLS];
                    {
                        let row = grid.get_mut(index).unwrap();
                        let start_index = iterations % row.len();

                        let mut insert_index = 0;
                        for i in start_index..(row.len() + start_index) {
                            new_row[i % row.len()] = row.get(insert_index).unwrap();
                            insert_index += 1;
                        }
                    }

                    grid[index] = new_row;
                }
            },
            _ => {},
        }
    }
}

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(err) => panic!("{:?}", err),
    };

    let mut grid: Grid = [["."; COLS]; ROWS];
    for line in text.lines() {
        let command = Command::new(line);
        command.apply(&mut grid);
    }

    let count = grid.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, &cell| {
            if cell == "#" {
                acc + 1
            } else {
                acc
            }
        })
    });
    println!("{}", count);
}
