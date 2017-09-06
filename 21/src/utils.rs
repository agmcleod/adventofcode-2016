pub fn rotate_vec_right(data: &mut Vec<u8>) {
    let copy = data.clone();
    for (i, v) in copy.iter().enumerate() {
        let index = if i == data.len() - 1 {
            0
        } else {
            i + 1
        };
        data[index] = *v;
    }
}

pub fn rotate_vec_left(data: &mut Vec<u8>) {
    let copy = data.clone();
    for (i, v) in copy.iter().enumerate() {
        let index = if i == 0 {
            data.len() - 1
        } else {
            i - 1
        };
        data[index] = *v;
    }
}

pub fn get_two_numbers(line: &str) -> Vec<usize> {
    line.split_whitespace().filter(|&n| {
        match n.parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }).map(|num| num.parse::<usize>().unwrap()).collect()
}