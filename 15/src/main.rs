fn main() {
    // positions, target position, current position
    // target position is 0 index - discs index
    // Essentially the diff it will need to be from current time of the loop, in order to hit 0
    let discs = [
        [13, 12, 1],
        [19, 17, 10],
        [3, 0, 2],
        [7, 3, 1],
        [5, 0, 3],
        [17, 11, 5],
        [11, 4, 0], // p2
    ];

    let mut time = 0;
    loop {
        time += 1;
        let result = discs.iter().fold(1, |acc, &disc| {
            acc & if (disc[2] + time) % disc[0] == disc[1] {
                1
            } else {
                0
            }
        });
        if result == 1 {
            println!("{}", time);
            break
        }
    }
}
