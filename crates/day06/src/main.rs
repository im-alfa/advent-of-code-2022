use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let a = include_bytes!("../input.txt")
        .windows(14)
        .position(|w| w.iter().collect::<std::collections::HashSet<_>>().len() == 14)
        .unwrap() + 14;

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);

    dbg!(a);
}
