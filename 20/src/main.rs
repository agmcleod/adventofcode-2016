extern crate read_input;
use read_input::read_text;

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut ranges: Vec<[u32; 2]> = Vec::new();
    let mut low_values: Vec<u32> = Vec::new();
    for line in text.lines() {
        let mut new_range = [0u32; 2];
        for (i, part) in line.split("-").enumerate() {
            new_range[i] = part.parse().expect("thing");
        }
        if new_range[0] == 0 {
            low_values.push(0);
        } else {
            low_values.push(new_range[0] - 1);
        }
        ranges.push(new_range);
    }

    low_values.sort();

    for value in &low_values {
        let mut invalid = false;
        for range in &ranges {
            if *value >= range[0] && *value <= range[1] {
                invalid = true;
                break
            }
        }
        if !invalid {
            println!("{}", value);
            break
        }
    }

    let mut count = 0;
    let mut last_number = 0;

    'main: for value in &low_values {
        for range in &ranges {
            if range[0] == value + 1 || range[0] == 0 {
                if range[0] > last_number + 1 {
                    count += range[0] - last_number - 1;
                }
                if range[1] > last_number {
                    last_number = range[1];
                    if last_number == <u32>::max_value() {
                        break 'main;
                    }
                }
            }
        }
    }

    println!("count: {}", count);
}
