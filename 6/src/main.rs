extern crate read_input;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(t) => t,
        Err(err) => panic!("{:?}", err),
    };

    let line_count = text.lines().count();

    let mut letter_counts: [HashMap<&str, usize>; 8] = [
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count),
        HashMap::with_capacity(line_count)
    ];

    for line in text.lines() {
        for (i, ch) in line.split("").enumerate() {
            if ch == "" {
                continue
            }

            let mut set = letter_counts.get_mut(i - 1).unwrap();
            if set.contains_key(ch) {
                let mut n = set.get_mut(ch).unwrap();
                *n += 1;
            } else {
                set.insert(ch, 1);
            }
        }
    }

    let mut result: [&str; 8] = [""; 8];
    let mut result_p2: [&str; 8] = [""; 8];
    for (i, col) in letter_counts.iter().enumerate() {
        let mut max_count = 0;
        let mut min_count = 10000;

        let mut highest_char = "";
        let mut min_char = "";

        for (ch, count) in col {
            max_count = cmp::max(max_count, *count);
            if max_count == *count {
                highest_char = ch;
            }

            min_count = cmp::min(min_count, *count);
            if min_count == *count {
                min_char = ch;
            }
        }

        result[i] = highest_char;
        result_p2[i] = min_char;
    }

    println!("{}", result.join(""));
    println!("{}", result_p2.join(""));
}
