extern crate itertools;

use itertools::Itertools;
use std::collections::VecDeque;

const ELF_COUNT: usize = 3001330;

fn p1() {
    let mut presents = vec![1; ELF_COUNT];
    let mut index = 0;
    loop {
        if presents[index] != 0 {
            let mut offset = 1;
            loop {
                if presents[(index + offset) % ELF_COUNT] > 0 {
                    presents[index] += presents[(index + offset) % ELF_COUNT];
                    presents[(index + offset) % ELF_COUNT] = 0;
                    break
                }
                offset += 1;
            }

            if presents[index] == ELF_COUNT {
                println!("{}", index + 1);
                break
            }
        }

        index += 1;
        if index >= ELF_COUNT {
            index = 0;
        }
    }
}

fn step_p2(v: Vec<usize>) -> Vec<usize> {
    let mut r = Vec::with_capacity(v.len() * 8 / 9);
    let first_high = (v.len() + 2) / 3;
    let after_last_high = v.len() / 2;
    let first_inc = 2 - v.len() % 2;
    for &e in v[first_high..after_last_high].iter() {
        r.push(e);
    }
    for i in (after_last_high + first_inc..v.len()).step(3) {
        r.push(v[i]);
    }
    for &e in v[..first_high].iter() {
        r.push(e);
    }
    r
}

fn p2() {
    let mut v: Vec<usize> = (1..(ELF_COUNT + 1)).collect();
    while v.len() > 1 {
        v = step_p2(v);
    }
    println!("{}", v[0]);
}

fn main() {
    // p1();
    p2();
}
