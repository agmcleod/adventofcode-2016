extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn password_two_complete(pw: &[char; 8]) -> bool {
    pw[0] != '+' &&
    pw[1] != '+' &&
    pw[2] != '+' &&
    pw[3] != '+' &&
    pw[4] != '+' &&
    pw[5] != '+' &&
    pw[6] != '+' &&
    pw[7] != '+'
}

fn main() {
    let mut n = 0;
    let mut password = String::new();
    let mut password_two = ['+'; 8];
    let mut md5 = Md5::new();

    loop {
        let iteration = format!("ugkcyxxp{:?}", n);
        n += 1;
        md5.input(iteration.as_bytes());
        let mut result = md5.result_str();
        md5.reset();

        if result.starts_with("00000") {
            result = result.replace("00000", "");

            let mut chars = result.chars();

            let first_number = chars.nth(0).unwrap();
            if password.len() < 8 {
                password.push(first_number);
            }

            let index: usize = match first_number.to_string().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            if index < 8 && password_two[index] == '+' {
                // grab first again, because mutable nth moves to next
                password_two[index] = chars.nth(0).unwrap();

                if password.len() >= 8 && password_two_complete(&password_two) {
                    break
                }
            }
        }
    }

    println!("{}", password);
    println!("{:?}", password_two);
}
