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
            _ => (acc.0, set),
        }
    });

    let set = result.1;
    set[0] != set[1] && set[0] == set[3] && set[1] == set[2]
}

fn check_text_for_bab(text: &str, aba: &[&str; 3]) -> bool {
    let result = text.split("").filter(|ch| *ch != "").fold((0, [""; 3]), |acc, ch| {
        let mut set = acc.1;
        match acc.0 {
            0 => {
                if ch == aba[1] {
                    set[0] = ch;
                    (1, set)
                } else {
                    (0, set)
                }
            },
            1 => {
                if ch == aba[0] {
                    set[1] = ch;
                    (2, set)
                } else if ch == aba[1] {
                    set[0] = ch;
                    (1, set)
                } else {
                    (0, set)
                }
            },
            2 => {
                if ch == aba[1] {
                    set[2] = ch;
                    (3, set)
                } else {
                    (0, set)
                }
            },
            _ => (acc.0, set),
        }
    });

    let set = result.1;
    aba[0] == set[1] && aba[1] == set[0] && aba[1] == set[2]
}

fn check_text_for_aba(text: &str) -> Vec<[&str; 3]> {
    let mut results = Vec::with_capacity(15); // arbitrary capacity
    text.split("").filter(|ch| *ch != "").fold((0, [""; 3]), |acc, ch| {
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
                if set[0] == ch {
                    set[2] = ch;
                    results.push(set);
                    let new_set = [set[1], set[2], ""];
                    (2, new_set)
                } else if set[1] != ch {
                    set[0] = set[1];
                    set[1] = ch;
                    (2, set)
                } else {
                    set[0] = ch;
                    (1, set)
                }
            },
            _ => (acc.0, set),
        }
    });

    results
}

fn valid_ssl(address: &str) -> bool {
    let re = regex::Regex::new(r"\[|\]").unwrap();
    let mut iter = 0;
    let mut valid = false;
    let mut aba_results: Vec<[&str; 3]> = Vec::new();
    for part in re.split(address) {
        if iter % 2 == 0 {
            aba_results = check_text_for_aba(part);
            if aba_results.len() > 0 {
                break
            }
        }
        iter += 1;
    }

    if aba_results.len() == 0 {
        return false
    }

    iter = 0;
    for part in re.split(address) {
        if iter % 2 != 0 {
            for aba in aba_results.iter() {
                if check_text_for_bab(part, &aba) {
                    valid = true;
                    break
                }
            }
            if valid {
                break
            }
        }
        iter += 1;
    }

    valid
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
    let mut ssl_count = 0;
    for line in text.lines() {
        if valid_tls(line) {
            count += 1;
        }
        if valid_ssl(line) {
            ssl_count += 1;
        }
    }

    println!("{}", count);
    println!("{}", ssl_count);
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

#[test]
fn test_check_text_for_aba() {
    assert_eq!(check_text_for_aba("aba"), vec![["a", "b", "a"]]);
    assert_eq!(check_text_for_aba("aaba"), vec![["a", "b", "a"]]);

    assert_eq!(check_text_for_aba("aabbaa"), Vec::<[&str; 3]>::new());
    assert_eq!(check_text_for_aba("aaa"), Vec::<[&str; 3]>::new());

    assert_eq!(check_text_for_aba("xyzyzabte"), vec![["y", "z", "y"], ["z", "y", "z"]]);

    assert_eq!(check_text_for_aba("aacacdef"), vec![["a", "c", "a"], ["c", "a", "c"]]);

    assert_eq!(check_text_for_aba("babaca"), vec![["b", "a", "b"], ["a", "b", "a"], ["a", "c", "a"]]);

    assert_eq!(check_text_for_aba("uhsjyhwppwarmrfed"), vec![["r", "m", "r"]]);
}

#[test]
fn test_check_text_for_bab() {
    let aba = ["b", "a", "b"];
    assert_eq!(check_text_for_bab("aba", &aba), true);

    let aba = ["b", "a", "b"];
    assert_eq!(check_text_for_bab("bab", &aba), false);

    let aba = ["b", "a", "b"];
    assert_eq!(check_text_for_bab("caba", &aba), true);

    let aba = ["b", "a", "b"];
    assert_eq!(check_text_for_bab("xxyzazabaii", &aba), true);
}

#[test]
fn test_valid_ssl() {
    assert_eq!(valid_ssl("aabb[xyx]yxy[abcd]efgh"), true);
    assert_eq!(valid_ssl("aba[aba]yxy[abcd]efgh"), false);
    assert_eq!(valid_ssl("bab[aba]yxy[abcd]efgh"), true);
    assert_eq!(valid_ssl("bab[xxx]yxy[aba]efgh"), true);
    assert_eq!(valid_ssl("bab[xxx]yxy[ccabafe]efgh"), true);
    assert_eq!(valid_ssl("xibabef[xxx]yxy[ccabafe]efgh"), true);
    assert_eq!(valid_ssl("aabb[xaxyxyu]rxy[abcd]utyxyioq"), true);
    assert_eq!(valid_ssl("aabb[bbcdef]rtotefd[erghotoew]"), true);
    assert_eq!(valid_ssl("zazbz[bzb]cdb"), true);
}