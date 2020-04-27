use rand::Rng;
use std::io::{stdin,stdout,Write};

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    WIN,
    LOSE,
    DRAW,
}

#[derive(PartialEq, Eq, Debug)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Move {
    fn from_u32(value: u32) -> Move {
        match value {
            1 => Move::ROCK,
            2 => Move::PAPER,
            3 => Move::SCISSORS,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

#[derive(Debug)]
struct Round {
    player_move: Move,
    cpu_move: Move,
}

impl Round {
    fn player_outcome(&self) -> Outcome {
        if self.player_move.eq(&self.cpu_move) {
            return Outcome::DRAW;
        }

        if self.player_move.eq(&Move::ROCK) && self.cpu_move.eq(&Move::SCISSORS) {
            return Outcome::WIN;
        }

        if self.player_move.eq(&Move::PAPER) && self.cpu_move.eq(&Move::ROCK) {
            return Outcome::WIN;
        }

        if self.player_move.eq(&Move::SCISSORS) && self.cpu_move.eq(&Move::PAPER) {
            return Outcome::WIN;
        }

        return Outcome::LOSE;
    }
}

fn cpu_move() -> Move {
    let mut rng = rand::thread_rng();
    let i: u32 = rng.gen_range(1, 3);
    return Move::from_u32(i);
}

fn player_move() -> Move {
    let mut s = String::new();
    println!("Please Select: ");
    println!("\t1: ROCK");
    println!("\t2: PAPER");
    println!("\t3: SCISSORS");
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s.retain(|c| !c.is_whitespace());

    let choice: u32 = s.parse::<u32>().unwrap();
    return Move::from_u32(choice);
}

fn game() -> Round {
    let round = Round {
        cpu_move: cpu_move(),
        player_move: player_move(),
    };

    return round;
}

fn main() {
    loop {
        let round = game();
        println!("{:?} v {:?}: {:?}", round.player_move, round.cpu_move, round.player_outcome());
    }
}
