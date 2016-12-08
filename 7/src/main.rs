extern crate read_input;
extern crate regex;

fn check_text_for_abba(text: &str) -> bool {
    let result = text.split("").filter(|ch| *ch != "").fold((0, [""; 4]), |acc, ch| {
        let mut set = acc.1;
        match acc.0 {
            0 => {
                set[0] = ch;
                (1, set)
            },
            1 => {
                if set[0] == ch {
                    (1, set)
                } else {
                    set[1] = ch;
                    (2, set)
                }
            },
            2 => {
                if set[1] == ch {
                    set[2] = ch;
                    (3, set)
                } else {
                    set[0] = set[1];
                    set[1] = ch;
                    (2, set)
                }
            },
            3 => {
                if set[0] == ch {
                    set[3] = ch;
                    (4, set)
                } else {
                    set[0] = set[2];
                    set[1] = ch;
                    (2, set)
                }
            },
            _ => {
                (acc.0, set)
            },
        }
    });

    let set = result.1;
    set[0] != set[1] && set[0] == set[3] && set[1] == set[2]
}

fn valid_tls(address: &str) -> bool {
    let re = regex::Regex::new(r"\[|\]").unwrap();
    let mut iter = 0;
    let mut valid = false;
    for part in re.split(address) {
        let has_abba = check_text_for_abba(part);
        if iter % 2 != 0 {
            // break because one abba in square brackets is failure
            if has_abba {
                valid = false;
                break
            }
        } else if !valid && has_abba {
            valid = true;
        }
        iter += 1;
    }

    valid
}

fn main() {
    let text = match read_input::read_text("input.txt") {
        Ok(t) => t,
        Err(err) => panic!("{:?}", err),
    };

    let mut count = 0;
    for line in text.lines() {
        if valid_tls(line) {
            count += 1;
        }
    }

    println!("{}", count);
}

#[test]
fn test_check_text_for_abba() {
    assert_eq!(check_text_for_abba("abba"), true);
    assert_eq!(check_text_for_abba("aaaa"), false);
    assert_eq!(check_text_for_abba("aabbaa"), true);
    assert_eq!(check_text_for_abba("xyzzyabte"), true);
    assert_eq!(check_text_for_abba("xxyzzyabe"), true);
    assert_eq!(check_text_for_abba("abcdeffeal"), true);
    assert_eq!(check_text_for_abba("uhsjyhwppwarmrfed"), true);
}