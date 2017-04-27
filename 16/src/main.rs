const DISK_SIZE: usize = 35651584;

fn check_sum(value: String) -> String {
    let mut checksum_value = value.clone();
    loop {
        if checksum_value.len() % 2 != 0 {
            return checksum_value;
        }

        let copy_value = checksum_value.clone();
        let values = copy_value.split("").filter(|v| *v != "").collect::<Vec<&str>>();
        let values = values.chunks(2).map(|pair| {
            if pair[0] == pair[1] {
                "1"
            } else {
                "0"
            }
        }).collect::<Vec<&str>>();
        checksum_value = values.join("");
    }
}

fn dragon_curve(value: String) -> String {
    let mut second_part = value.chars().map(|v| {
        if v == '0' {
            String::from("1")
        } else {
            String::from("0")
        }
    }).collect::<Vec<String>>();
    second_part.reverse();

    format!("{}0{}", value, second_part.join(""))
}

fn main() {
    let mut sequence = String::from("10111011111001111");
    loop {
        if sequence.len() < DISK_SIZE {
            sequence = dragon_curve(sequence);
        } else {
            break
        }
    }

    let value = sequence[0..DISK_SIZE].to_string();
    println!("{}", check_sum(value));
}
