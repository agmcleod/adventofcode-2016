extern crate read_input;

use std::collections::HashMap;

struct Room {
    hash: String,
    id: usize,
    letters_by_count: HashMap<usize, Vec<String>>,
    name: String,
}

static LETTERS: [&'static str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"
];

impl Room {
    fn new(line: &str) -> Room {
        let mut letter_count_map: HashMap<String, usize> = HashMap::new();
        let room_encoding: Vec<&str> = line.split("[").collect();
        let pieces: Vec<&str> = room_encoding[0].split("-").collect();

        let name_chunks: Vec<&str> = pieces[0..pieces.len() - 1].to_vec();
        for piece in name_chunks {
            let characters: Vec<_> = piece.chars().collect();
            for character in characters {
                let letter = character.to_string();
                if letter_count_map.contains_key(&letter) {
                    let mut n = letter_count_map.get_mut(&letter).unwrap();
                    *n += 1;
                } else {
                    letter_count_map.insert(letter, 1);
                }
            }
        }

        let mut letters_by_count: HashMap<usize, Vec<String>> = HashMap::new();

        for (letter, count) in letter_count_map {
            if letters_by_count.contains_key(&count) {
                letters_by_count.get_mut(&count).unwrap().push(letter);
            } else {
                letters_by_count.insert(count, vec![letter]);
            }
        }

        let keys: Vec<usize> = letters_by_count.keys().cloned().collect();
        for n in keys {
            letters_by_count.get_mut(&n).unwrap().sort();
        }

        let hash: Vec<String> = room_encoding[1].chars().filter(|a| *a != ']').map(|v| v.to_string()).collect();

        let id: usize = pieces[pieces.len() - 1].parse().unwrap();

        let room = Room {
            hash: hash.join(""),
            id: id,
            letters_by_count: letters_by_count,
            name: pieces[0..pieces.len() - 1].join(""),
        };

        room
    }

    fn is_real(self: &Room) -> bool {
        let mut keys: Vec<usize> = self.letters_by_count.keys().cloned().collect();
        keys.sort_by(|a, b| {
            b.cmp(a)
        });

        let mut count_in_hash = 0;
        for n in keys {
            let letters = self.letters_by_count.get(&n).unwrap();
            let mut count_used = 0;
            let mut pattern = String::new();

            for (i, l) in letters.iter().enumerate() {
                if i >= 5 || i >= self.hash.len() - count_in_hash {
                    break
                }
                pattern.push_str(l);
            }

            if self.hash.contains(&pattern) {
                count_in_hash += pattern.len();
                count_used += pattern.len();
            }

            if count_in_hash == self.hash.len() {
                return true
            } else if letters.len() < count_used {
                return false
            }
        }

        return false
    }

    fn decrypt(self: &Room) -> String {
        let mut result = String::new();
        let letters: Vec<String> = self.name.replace("-", "").chars().map(|v| v.to_string()).collect();
        for letter in letters {
            let index = LETTERS.iter().position(|&v| v == letter).unwrap();
            result.push_str(LETTERS[(index + self.id) % LETTERS.len()]);
        }

        result
    }
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(text) => text,
        Err(err) => panic!("{:?}", err),
    };

    let mut count = 0;
    for line in text.lines() {
        let room = Room::new(line);
        if room.is_real() {
            count += room.id;
            let name = room.decrypt();
            if name.contains("north") && name.contains("pole") {
                println!("north pole room: {}", room.id);
            }
        }
    }

    println!("{}", count);
}
