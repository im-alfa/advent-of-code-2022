use std::time::Instant;

#[derive(Debug)]
enum Move {
    Paper,
    Scissors,
    Rock
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Tie
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Outcome {
    fn from_moves(game: Vec<Move>) -> Self {
        if game.len() != 2 {
            panic!("This shouldn't have happened :'(")
        }

        let (move1, move2) = (game.get(0).unwrap(), game.get(1).unwrap());

        match (move1, move2) {
            (Move::Paper, Move::Paper) => Outcome::Tie,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Rock, Move::Rock) => Outcome::Tie,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Scissors, Move::Scissors) => Outcome::Tie,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
        }

    }
}


fn main() {
    let start_time = Instant::now();

    let input = include_str!("../input.txt");
    let games = input.lines();

    for game in games {
        let cases: Vec<Move> = game.split(" ").map(|case| Move::from_str(case)).collect();
        let result = Outcome::from_moves(cases);

        println!("Case 1: {:?}", result)
    }

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}
