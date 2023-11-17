use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let a = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (l1, l2) = line.split_once(",").unwrap();
            let (a, b) = l1.split_once("-").unwrap();
            let (c, d) = l2.split_once("-").unwrap();

            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a1, a2, b1, b2)| (a1 >= b1 && a2 <= b2) || (a1 <= b1 && a2 >= b2))
        .count();

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}
