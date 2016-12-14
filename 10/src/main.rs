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

#[derive(Debug)]
struct Instruction<'a> {
    num: usize,
    low_index: usize,
    high_index: usize,
    low_type: &'a str,
    high_type: &'a str,
}

impl<'a> Instruction<'a> {
    fn new(num: usize, low_index: usize, high_index: usize, low_type: &'a str, high_type: &'a str) -> Instruction<'a> {
        Instruction{ num: num, low_index: low_index, high_index: high_index, low_type: low_type, high_type: high_type }
    }
}

type BotMap = HashMap<usize, Bot>;
type Instructions<'a> = Vec<Instruction<'a>>;
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

fn build_bot_operation(bot: &mut Bot, outputs: &mut OutputMap, instruction: &Instruction) -> [[usize; 2]; 2] {
    let mut operations = [[0; 2]; 2];
    let low = bot.take_chip("low");
    let high = bot.take_chip("high");

    if instruction.low_type == "bot" {
        operations[0] = [instruction.low_index, low];
    } else {
        let mut output = outputs.get_mut(&instruction.low_index).unwrap();
        insert_chip_into_output(&mut output, low);
    }

    if instruction.high_type == "bot" {
        operations[1] = [instruction.high_index, high];
    } else {
        let mut output = outputs.get_mut(&instruction.high_index).unwrap();
        insert_chip_into_output(&mut output, low);
    }

    operations
}

fn solve(instructions: &Instructions, bots: &mut BotMap, outputs: &mut OutputMap) {
    let target_values = [61, 17];
    let mut index = 0;
    let mut found_p1 = false;
    let mut found_p2 = false;
    loop {
        let instruction = instructions.get(index % instructions.len()).unwrap();
        index += 1;

        let mut operations = [[0; 2]; 2];
        {
            let mut bot = match bots.get_mut(&instruction.num) {
                Some(b) => b,
                None => continue,
            };

            if bot.has_two() {
                operations = build_bot_operation(&mut bot, outputs, &instruction);
            }
        }

        let zero = 0;
        let one = 1;
        let two = 2;
        let first = outputs.get(&zero).unwrap()[0];
        let second = outputs.get(&one).unwrap()[0];
        let third = outputs.get(&two).unwrap()[0];
        if first != 0 && second != 0 && third != 0 {
            println!("{}", first * second * third);
            found_p2 = true;
        }

        let mut bots_to_test: [Option<&Bot>; 2] = [None, None];

        if operations[0][1] != 0 {
            give_to_bot(bots, &operations[0][0], operations[0][1]);
        }
        if operations[1][1] != 0 {
            give_to_bot(bots, &operations[1][0], operations[1][1]);
        }

        if operations[0][1] != 0 {
            bots_to_test[0] = bots.get(&instruction.low_index);
        }
        if operations[1][1] != 0 {
            bots_to_test[1] = bots.get(&instruction.low_index);
        }

        for (i, b) in bots_to_test.iter().enumerate() {
            if let Some(bot_to_test) = *b {
                if (bot_to_test.chips[0] == target_values[0] && bot_to_test.chips[1] == target_values[1]) ||
                (bot_to_test.chips[0] == target_values[1] && bot_to_test.chips[1] == target_values[0]) {
                    if i == 0 {
                        println!("p1 bot: {}", instruction.low_index);
                    } else {
                        println!("p1 bot: {}", instruction.high_index);
                    }
                    found_p1 = true;

                    break
                }
            }
        }

        if found_p1 && found_p2 {
            break
        }
    }
}

fn build_instructions<'a>(instructions: &mut Instructions<'a>, outputs: &mut OutputMap, text: &'a String) {
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

            instructions.push(Instruction::new(index, low_index, high_index, low_type, high_type));
        }
    }
}

fn main() {
    let text = match read_text("input.txt") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let mut instructions: Instructions = Instructions::with_capacity(text.lines().count());
    let mut outputs: OutputMap = HashMap::new();
    let mut bots: BotMap = HashMap::new();
    map_bots(&mut bots, &text);
    build_instructions(&mut instructions, &mut outputs, &text);
    solve(&instructions, &mut bots, &mut outputs);
}
