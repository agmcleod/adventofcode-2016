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

    fn has_two(self: &Bot) -> bool {
        self.chips[0] != 0 && self.chips[1] != 0
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

struct Instruction<'a> {
    low_index: usize,
    high_index: usize,
    low_type: &'a str,
    high_type: &'a str,
}

impl<'a> Instruction<'a> {
    fn new(low_index: usize, high_index: usize, low_type: &'a str, high_type: &'a str) -> Instruction<'a> {
        Instruction{ low_index: low_index, high_index: high_index, low_type: low_type, high_type: high_type }
    }
}

type BotMap = HashMap<usize, Bot>;
type InstructionMap<'a> = HashMap<usize, Instruction<'a>>;
type OutputMap = HashMap<usize, [usize; 20]>;

fn give_to_bot(bots: &mut BotMap, index: &usize, value: usize) {
    let mut insert_new = false;
    match bots.get_mut(index) {
        Some(receiver) => {
            receiver.give_chip(value);
        },
        None => insert_new = true,
    }

    if insert_new {
        new_bot_with_chip(bots, *index, value);
    }
}

fn map_bots(bots: &mut BotMap, text: &String) {
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

fn new_bot_with_chip(bots: &mut BotMap, index: usize, num: usize) {
    let mut bot = Bot::new();
    bot.give_chip(num);
    bots.insert(index, bot);
}

fn map_instructions<'a>(instructions: &'a mut InstructionMap<'a>, outputs: &'a mut OutputMap, text: &'a String) {
    for line in text.lines() {
        let mut words = line.split(" ");
        let line_type = words.next();
        if line_type == Some("bot") {
            let index: usize = match words.next() {
                Some(n) => n.parse().expect("2nd value should be a number"),
                None => panic!("No value found for instruction index"),
            };

            let low_type = match words.nth(3) {
                Some(t) => t,
                None => panic!("No value found for low type (bot|output)"),
            };

            let low_index: usize = match words.next() {
                Some(n) => n.parse().expect("low value should be a number"),
                None => panic!("No value found for \"low give to\" index"),
            };

            if low_type == "output" {
                outputs.insert(low_index, [0; 20]);
            }

            let high_type = match words.nth(3) {
                Some(t) => t,
                None => panic!("No value found for high type (bot|output)"),
            };

            let high_index: usize = match words.next() {
                Some(n) => n.parse().expect("high value should be a number"),
                None => panic!("No value found for \"high give to\" index"),
            };

            if high_type == "output" {
                outputs.insert(high_index, [0; 20]);
            }

            if instructions.contains_key(&index) {
                panic!("Instruction already contains one for bot {}", index);
            }
            instructions.insert(index, Instruction::new(low_index, high_index, low_type, high_type));
        }
    }
}

fn insert_chip_into_output(output: &mut[usize; 20], val: usize) {
    let mut it = output.iter_mut();
    loop {
        match it.next() {
            Some(n) => {
                if *n == 0 {
                    *n = val;
                    break
                }
            },
            None => break
        }
    }
}

fn operate_two_chip_bots(bots: &mut BotMap, outputs: &mut OutputMap, instructions: &InstructionMap) {
    let mut bot_operations: Vec<[usize; 2]> = Vec::with_capacity(bots.len());
    for (giver_index, bot) in bots.iter_mut() {
        if bot.has_two() {
            let low = bot.take_chip("low");
            let high = bot.take_chip("high");
            let instruction = match instructions.get(giver_index) {
                Some(instruction) => instruction,
                None => panic!("No instruction for {}", giver_index),
            };
            if instruction.low_type == "bot" {
                bot_operations.push([instruction.low_index, low]);
            } else {
                println!("{}, {:?}", instruction.low_index, outputs);
                let mut output = outputs.get_mut(&instruction.low_index).unwrap();
                insert_chip_into_output(&mut output, low);
            }

            if instruction.high_type == "bot" {
                bot_operations.push([instruction.high_index, high]);
            } else {
                println!("{}, {:?}", instruction.high_index, outputs);
                let mut output = outputs.get_mut(&instruction.high_index).unwrap();
                insert_chip_into_output(&mut output, low);
            }
        }
    }

    for operation in bot_operations {
        if operation[0] != 0 && operation[1] != 0 {
            give_to_bot(bots, &operation[0], operation[1]);
        }
    }
}

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut bots: BotMap = HashMap::new();
    let mut outputs: OutputMap = HashMap::new();
    let mut instructions: InstructionMap = HashMap::new();
    map_bots(&mut bots, &text);
    map_instructions(&mut instructions, &mut outputs, &text);

    let target_values = [61, 17];
    let mut found_bot = false;
    loop {
        operate_two_chip_bots(&mut bots, &mut outputs, &instructions);
        for (i, bot) in bots.iter_mut() {
            if (bot.chips[0] == target_values[0] && bot.chips[1] == target_values[1]) || (bot.chips[0] == target_values[1] && bot.chips[1] == target_values[0]) {
                println!("{}", i);
                found_bot = true;
                break
            }
        }

        if found_bot {
            break
        }
    }
}
