extern crate time;

fn get_raw_or_registered_num(value: &str, a: &i32, b: &i32, c: &i32, d: &i32) -> i32 {
    match value.parse() {
        Ok(v) => v,
        Err(e) => {
            match value {
                "a" => *a,
                "b" => *b,
                "c" => *c,
                "d" => *d,
                _ => panic!("{:?} - {}", e, value),
            }
        }
    }
}

fn process_input(input: &mut Vec<String>, a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut d = d;
    let mut start_index: i32 = 0;
    loop {
        let mut input_clone = input.clone();
        {
            let mut it = input.iter().skip(start_index as usize);
            while let Some(line) = it.next() {
                println!("Run: {} on {}", line, start_index);
                let mut cmd_it = line.split(" ");
                start_index += 1;
                match cmd_it.next() {
                    Some("cpy") => {
                        let value = cmd_it.next().unwrap();
                        let value = get_raw_or_registered_num(value, &a, &b, &c, &d);

                        let target = cmd_it.next().unwrap();
                        match target {
                            "a" => a = value,
                            "b" => b = value,
                            "c" => c = value,
                            "d" => d = value,
                            _ => continue,
                        }
                    },
                    Some("inc") => {
                        let target = cmd_it.next().unwrap();
                        match target {
                            "a" => a += 1,
                            "b" => b += 1,
                            "c" => c += 1,
                            "d" => d += 1,
                            _ => panic!("Register not found: {}", target),
                        }
                    },
                    Some("dec") => {
                        let target = cmd_it.next().unwrap();
                        match target {
                            "a" => a -= 1,
                            "b" => b -= 1,
                            "c" => c -= 1,
                            "d" => d -= 1,
                            _ => panic!("Register not found: {}", target),
                        }
                    },
                    Some("tgl") => {
                        let value = cmd_it.next().unwrap();
                        let offset = get_raw_or_registered_num(value, &a, &b, &c, &d);
                        let mut target_index = start_index - 1;
                        target_index += offset;
                        let target_index = (target_index as usize) % input.len();

                        let line = match input.get(target_index) {
                            Some(l) => l,
                            None => panic!("Could not get nth from input at: {}", target_index),
                        };
                        let count = line.split(" ").count();
                        cmd_it = line.split(" ");
                        let mut replace;
                        if count > 2 {
                            let command = match cmd_it.next() {
                                Some(c) => c,
                                None => panic!("no command"),
                            };
                            if command == "jnz" {
                                let from_value = cmd_it.next().unwrap();
                                let to_value = cmd_it.next().unwrap();
                                replace = format!("cpy {} {}", from_value, to_value);
                            } else {
                                replace = format!("jnz {} {}", cmd_it.next().unwrap(), cmd_it.next().unwrap());
                            }
                        } else {
                            let command = cmd_it.next().unwrap();
                            if command == "inc" {
                                replace = "dec".to_string();
                            } else {
                                replace = "inc".to_string();
                            }

                            replace = format!("{} {}", replace, cmd_it.next().unwrap());
                        }

                        input_clone[target_index] = replace;
                        break
                    },
                    Some("jnz") => {
                        let value = cmd_it.next().unwrap();
                        let value = get_raw_or_registered_num(value, &a, &b, &c, &d);

                        if value != 0 {
                            let offset = get_raw_or_registered_num(cmd_it.next().unwrap(), &a, &b, &c, &d);
                            if offset == 0 {
                                continue
                            }
                            println!("jnz {} {}", value, offset);
                            if offset > 0 {
                                for _ in 0..offset-1 {
                                    start_index += 1;
                                    it.next();
                                }
                            } else {
                                start_index += offset - 1;
                                break
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        *input = input_clone;

        if start_index as usize >= input.len() {
            break
        }
    }

    a
}

fn main() {
    let mut input = "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 75 c
jnz 72 d
inc a
inc d
jnz d -2
inc c
jnz c -5".lines().map(|v| v.to_string()).collect::<Vec<String>>();

    let mut input_two = input.clone();

    println!("{}", process_input(&mut input, 7, 0, 0, 0));
    println!("{}", process_input(&mut input_two, 12, 0, 0, 0));
}
