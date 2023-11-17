use std::time::Instant;
use std::path::MAIN_SEPARATOR;
use itertools::Itertools;

const STACKS: usize = 9;

fn main() {
    let start_time = Instant::now();

    let input = include_bytes!("../input.txt");
    let (stack, actions) = input.split_at(input.windows(2).position(|w| w == b"\n\n").unwrap() + 2);

    let mut stacks: [Vec<u8>; STACKS] = Default::default();

    stack.split(|c| c == &b'\n').rev().skip(1).for_each(|l| {
        l.iter()
            .skip(1) // skip first bracket
            .step_by(4)
            .enumerate() // jump to the next letter
            .filter(|(_, c)| c != &&b' ')// skip spaces
            .for_each(|(i, c)| stacks[i].push(*c)) // push to the stack
    });


    // move 1 from 2 to 1
    actions.split(|c| c == &b'\n').for_each(|l| {
        // store only the numbers in a tuple of usize (from, to, to_move)
        let (n, a, b): (usize, _, _) = l
            .split(|b| b == &b' ')
            .skip(1)
            .step_by(2)
            .map(|n| atoi::atoi(n).unwrap())
            .collect_tuple()
            .unwrap();

        for _ in 0..n {
            let tmp = stacks[a-1].pop().unwrap();
            stacks[b - 1].push(tmp);
        }
    });
    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);

    stacks.iter()
        .for_each(|s| print!("{}", *s.last().unwrap() as char));
    println!()
}
