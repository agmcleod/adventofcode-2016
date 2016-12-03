extern crate read_input;
extern crate regex;
use regex::Regex;

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err),
    };

    let lines = text.split("\n");
    let mut count = 0;
    let split_re = Regex::new(r"\s+").unwrap();
    for line in lines {
        if line == "" {
            continue
        }

        let mut numbers: Vec<usize> = split_re.split(line.trim()).map(|v| v.parse().unwrap()).collect();
        numbers.sort();
        if numbers[0] + numbers[1] > numbers[2] {
            count += 1;
        }
    }

    println!("{}", count);
}
