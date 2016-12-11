extern crate read_input;
extern crate regex;

use read_input::read_text;
use regex::Regex;

struct Marker {
    range: usize,
    repeat: usize,
}

impl Marker {
    fn new(text: &str) -> Marker {
        let mut values = text.split("x");
        let range = values.next().unwrap();
        let range: usize = (&range[1..]).parse().expect("First value should be a number");
        let repeat = values.next().unwrap();
        let repeat: usize = (&repeat[0..(repeat.len()-1)]).parse().expect("Second value should be a number");
        Marker {
            range: range,
            repeat: repeat,
        }
    }
}

fn next_marker<'a>(re: &Regex, text: &'a str) -> (&'a str, usize) {
    let capture = re.captures(text);

    let capture = match capture {
        Some(c) => c,
        None => {
            return ("", 0)
        },
    };

    let text = match capture.at(0) {
        Some(t) => t,
        None => "",
    };

    if text == "" {
        ("", 0)
    } else {
        let pos = match capture.pos(0) {
            Some(size) => size.1,
            None => 0,
        };

        (text, pos)
    }
}

fn decompress(text: String, re: &Regex, v2: bool) -> String {
    let mut text = text.as_ref();
    let mut transformed = String::new();
    loop {
        let result = next_marker(&re, text);
        if result.0 == "" {
            if text.len() <= 1 {
                transformed.push_str(text);
                break
            } else {
                transformed.push_str(&text[0..1]);
                text = &text[1..];
                continue
            }
        }

        let marker = Marker::new(result.0);
        let start_index = result.1;

        let mut sub = String::from(&text[start_index..(marker.range + start_index)]);

        if v2 && re.is_match(sub.as_ref()) {
            sub = decompress(sub, re, true);
        }

        let sub = sub.as_ref();

        let mut i = 0;
        while i < marker.repeat {
            transformed.push_str(sub);
            i += 1;
        }

        if marker.range + start_index >= text.len() {
            break
        }
        text = &text[(marker.range + start_index)..];
    }

    transformed
}

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(err) => panic!("{:?}", err),
    };

    let re = Regex::new(r"^\(\d+x\d+\)").unwrap();
    println!("{}", decompress(text.clone(), &re, false).len());
    println!("{}", decompress(text, &re, true).len());
}
