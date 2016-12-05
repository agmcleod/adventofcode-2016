extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut n = 0;
    let mut password = String::new();
    let mut password_two = String::new();
    let mut md5 = Md5::new();

    loop {
        let iteration = format!("ugkcyxxp{:?}", n);
        md5.input(iteration.as_bytes());
        let mut result = md5.result_str();

        if result.starts_with("00000") {
            result = result.replace("00000", "");
            password.push(result.chars().nth(0).unwrap());
            if password.len() >= 8 {
                break
            }
        }

        n += 1;

        md5.reset();
    }

    println!("{}", password);
}
