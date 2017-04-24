extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

const INPUT: &'static str = "jlmsuwbz";

fn get_md5(md5: &mut Md5, n: usize) -> String {
    let iteration = format!("{}{}", INPUT, n);

    md5.input(iteration.as_bytes());
    let result = md5.result_str();
    md5.reset();

    result
}

fn get_md5_p2(hashes: &mut HashMap<usize, String>, md5: &mut Md5, n: usize) -> String {
    if let Some(hash) = hashes.get(&n) {
        return hash.clone()
    }

    let mut iteration = format!("{}{}", INPUT, n);

    for _ in 0..2017 {
        md5.input(iteration.as_bytes());
        iteration = md5.result_str().to_lowercase();
        md5.reset();
    }

    hashes.insert(n, iteration.clone());

    iteration
}

fn main () {
    let mut n = 0;
    let mut md5 = Md5::new();

    let mut key_count = 0;
    let mut hashes: HashMap<usize, String> = HashMap::new();

    loop {
        let result = get_md5_p2(&mut hashes, &mut md5, n);

        let characters = result.split("").collect::<Vec<&str>>();
        let mut last_character = "";
        let mut character_to_check: Option<&str> = None;
        for (i, character) in characters.iter().enumerate() {
            if *character == last_character && i + 1 < characters.len() && characters[i + 1] == *character {
                character_to_check = Some(character.clone());
                break
            } else {
                last_character = character.clone();
            }
        }

        if let Some(character) = character_to_check {
            for i in 1..1000 {
                let result = get_md5_p2(&mut hashes, &mut md5, n + i);

                let five_chars = format!("{}{}{}{}{}", character, character, character, character, character);
                if result.contains(&five_chars) {
                    key_count += 1;
                    break
                }
            }

            if key_count == 64 {
                println!("result: {}", n);
                break
            }
        }

        n += 1;
    }
}