extern crate time;

fn get_number_from_token(value: &str) -> i32 {
    match value.parse() {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    }
}

fn get_raw_or_registered_num(value: &str, a: &i32, b: &i32, c: &i32, d: &i32) -> i32 {
    match value.parse() {
        Ok(v) => v,
        Err(e) => {
            match value {
                "a" => *a,
                "b" => *b,
                "c" => *c,
                "d" => *d,
                _ => panic!("{:?}", e),
            }
        }
    }
}

fn process_input(input: &str, a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut d = d;
    let mut start_index: i32 = 0;
    loop {
        let mut it = input.lines().skip(start_index as usize);
        while let Some(line) = it.next() {
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
                        _ => panic!("Register not found: {}", target),
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
                        let offset = get_number_from_token(cmd_it.next().unwrap());
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

        if start_index as usize >= input.lines().count() {
            break
        }
    }

    a
}

fn main() {
    let input = "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 19 c
cpy 11 d
inc a
dec d
jnz d -2
dec c
jnz c -5";



    let start = time::now().to_timespec();
    println!("{}", process_input(input, 0, 0, 0, 0));
    println!("{:?}", time::now().to_timespec() - start);

    let start = time::now().to_timespec();
    println!("{}", process_input(input, 0, 0, 1, 0));
    println!("{:?}", time::now().to_timespec() - start);
}
