use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let max = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .max();
    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
    dbg!(max);
}
