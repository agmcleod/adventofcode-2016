extern crate read_input;
extern crate regex;
use regex::Regex;

fn is_possible(numbers: &mut Vec<usize>) -> bool {
    numbers.sort();
    return numbers[0] + numbers[1] > numbers[2]
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err),
    };

    let lines = text.split("\n");
    let mut count = 0;
    let split_re = Regex::new(r"\s+").unwrap();

    let mut vertical_triangles = Vec::new();

    let mut line_count = 0;
    for line in lines {
        if line == "" {
            continue
        }

        let mut numbers: Vec<usize> = split_re.split(line.trim()).map(|v| v.parse().unwrap()).collect();

        if line_count % 3 == 0 {
            vertical_triangles.push(vec![numbers[0]]);
            vertical_triangles.push(vec![numbers[1]]);
            vertical_triangles.push(vec![numbers[2]]);
        } else {
            let idx = vertical_triangles.len() - 3;
            vertical_triangles[idx].push(numbers[0]);
            vertical_triangles[idx + 1].push(numbers[1]);
            vertical_triangles[idx + 2].push(numbers[2]);
        }

        if is_possible(&mut numbers) {
            count += 1;
        }

        line_count += 1;
    }

    println!("p1 {}", count);

    count = 0;
    for mut vertical_triangle in vertical_triangles.iter_mut() {
        if is_possible(&mut vertical_triangle) {
            count += 1;
        }
    }

    println!("p2 {}", count);
}
