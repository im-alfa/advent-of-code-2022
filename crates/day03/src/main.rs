use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let input = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)|
            b.iter().filter(|b| a.contains(b))
                .map(|c|
                    if *c >= b'a' {
                        (c - b'a') + 1
                    } else {
                        (c - b'A') + 27
                    }

                ).next().unwrap())
        .sum::<u8>();
    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}
