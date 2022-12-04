use std::collections::HashSet;
use Outcome::*;
use Shape::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Shape {
    fn from(move_str: &str) -> Self {
        match move_str {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unrecognized move_str {move_str}"),
        }
    }
}

impl Shape {
    fn counter(&self, outcome: Outcome) -> Shape {
        match self {
            Rock => match outcome {
                Win => Paper,
                Lose => Scissors,
                Draw => Rock,
            },
            Paper => match outcome {
                Win => Scissors,
                Lose => Rock,
                Draw => Paper,
            },
            Scissors => match outcome {
                Win => Rock,
                Lose => Paper,
                Draw => Scissors,
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(outcome: &str) -> Self {
        match outcome {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("You're killing mine, Smalls!"),
        }
    }
}

impl Outcome {
    fn get(their_move: Shape, my_move: Shape) -> Self {
        let mut wins = HashSet::new();
        wins.insert((Rock, Paper));
        wins.insert((Paper, Scissors));
        wins.insert((Scissors, Rock));
        let wins = wins;

        if wins.contains(&(their_move, my_move)) {
            Win
        } else if their_move == my_move {
            Draw
        } else {
            Lose
        }
    }
}

fn round_score(their_move: Shape, my_move: Shape) -> u32 {
    my_move as u32 + Outcome::get(their_move, my_move) as u32
}

fn main() {
    let in_file = std::env::args().skip(1).take(1).next().unwrap();
    let input = std::fs::read_to_string(in_file).unwrap();
    let total_score: u32 = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(theirs, mine)| (Shape::from(theirs), Shape::from(mine)))
        .map(|(theirs, mine)| round_score(theirs, mine))
        .sum();
    println!("total score: {total_score}");

    let total_score2: u32 = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(theirs, need)| (Shape::from(theirs), Outcome::from(need)))
        .map(|(theirs, need)| (theirs, theirs.counter(need)))
        .map(|(theirs, mine)| round_score(theirs, mine))
        .sum();
    println!("total score 2: {total_score2}");
}
