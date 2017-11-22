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

fn process_input(input: Vec<String>, a: i32, b: i32, c: i32, d: i32) -> bool {
    let start_a = a;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut d = d;
    let mut start_index: i32 = 0;
    let mut out_switch = 0;
    let mut count = 0;
    'main: loop {
        {
            let mut it = input.iter().skip(start_index as usize);
            while let Some(line) = it.next() {
                // println!("Run: {} on {}", line, start_index);
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
                    Some("jnz") => {
                        let value = cmd_it.next().unwrap();
                        let value = get_raw_or_registered_num(value, &a, &b, &c, &d);

                        if value != 0 {
                            let offset = get_raw_or_registered_num(cmd_it.next().unwrap(), &a, &b, &c, &d);
                            if offset == 0 {
                                continue
                            }
                            // println!("jnz {} {}", value, offset);
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
                    Some("out") => {
                        let value = cmd_it.next().unwrap();
                        let value = get_raw_or_registered_num(value, &a, &b, &c, &d);
                        if value != out_switch {
                            break 'main
                        }

                        count += 1;
                        if out_switch == 0 {
                            out_switch = 1;
                        } else {
                            out_switch = 0;
                        }
                        if count >= 20 {
                            println!("{} counts verified signal. Original value of: {}", 20, start_a);
                            return true
                        }
                    },
                    _ => {},
                }
            }
        }

        if start_index as usize >= input.len() {
            break
        }
    }

    false
}

fn main() {
    let mut input = "cpy a d
cpy 4 c
cpy 643 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21".lines().map(|v| v.to_string()).collect::<Vec<String>>();

    let mut a = 0;
    loop {
        if !process_input(input.clone(), a, 0, 0, 0) {
            a += 1;
        } else {
            break
        }
    }
}
