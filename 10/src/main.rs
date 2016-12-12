extern crate read_input;
use read_input::read_text;
use std::collections::HashMap;

struct Bot {
    chips: [usize; 2],
}

impl Bot {
    fn new() -> Bot {
        Bot{ chips: [0; 2] }
    }

    fn give_chip(self: &mut Bot, value: usize) {
        if self.chips[0] == 0 {
            self.chips[0] = value;
        } else if self.chips[1] == 0 {
            self.chips[1] = value;
        } else {
            panic!("Bot already has chips: {:?}", self.chips);
        }
    }

    fn take_chip(self: &mut Bot, which: &str) -> usize {
        let mut return_value = 0;
        if which == "low" {
            if self.chips[0] < self.chips[1] {
                return_value = self.chips[0];
                self.chips[0] = 0;
            } else {
                return_value = self.chips[1];
                self.chips[1] = 0;
            }
        } else if which == "high" {
            if self.chips[0] > self.chips[1] {
                return_value = self.chips[0];
                self.chips[0] = 0;
            } else {
                return_value = self.chips[1];
                self.chips[1] = 0;
            }
        }

        return_value
    }
}

struct Instruction {
    low_index: usize,
    high_index: usize,
}

impl Instruction {
    fn new(low_index: usize, high_index: usize) -> Instruction {
        Instruction{ low_index: low_index, high_index: high_index }
    }
}

fn give_to_bot(bots: &mut HashMap<usize, Bot>, index: &usize, value: usize) {
    let mut insert_new = false;
    match bots.get_mut(index) {
        Some(receiver) => receiver.give_chip(value),
        None => {
            insert_new = true;
        },
    }

    if insert_new {
        new_bot_with_chip(bots, *index, value);
    }
}

fn map_bots(bots: &mut HashMap<usize, Bot>, text: &String) {
    for line in text.lines() {
        let mut words = line.split(" ");
        let line_type = words.next();
        if line_type == Some("value") {
            let num: usize = words.next().unwrap().parse().expect("2nd value should be a number");
            if let Some(target) = words.nth(3) {
                let target: usize = target.parse().expect("Expected target to be a number");
                if bots.contains_key(&target) {
                    let mut bot = bots.get_mut(&target).unwrap();
                    bot.give_chip(num);
                } else {
                    new_bot_with_chip(bots, target, num);
                }
            }
        }
    }
}

fn new_bot_with_chip(bots: &mut HashMap<usize, Bot>, index: usize, num: usize) {
    let mut bot = Bot::new();
    bot.give_chip(num);
    bots.insert(index, bot);
}

fn map_instructions(instructions: &mut HashMap<usize, Instruction>, text: &String) {
    for line in text.lines() {
        let mut words = line.split(" ");
        let line_type = words.next();
        if line_type == Some("bot") {
            let index: usize = match words.next() {
                Some(n) => n.parse().expect("2nd value should be a number"),
                None => panic!("No value found for instruction index"),
            };

            let low_index: usize = match words.nth(4) {
                Some(n) => n.parse().expect("low value should be a number"),
                None => panic!("No value found for \"low give to\" index"),
            };

            let high_index: usize = match words.nth(4) {
                Some(n) => n.parse().expect("high value should be a number"),
                None => panic!("No value found for \"high give to\" index"),
            };

            if instructions.contains_key(&index) {
                panic!("Instruction already contains one for bot {}", index);
            }
            instructions.insert(index, Instruction::new(low_index, high_index));
        }
    }
}

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut instructions: HashMap<usize, Instruction> = HashMap::new();
    map_bots(&mut bots, &text);
    map_instructions(&mut instructions, &text);
}
