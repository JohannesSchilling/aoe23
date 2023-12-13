use std::io::{BufRead, BufReader, stdin};
use std::cmp::max;


#[derive(PartialEq, Eq, Debug)]
pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Round {
    r: usize,
    g: usize,
    b: usize,
}

impl Round {
    fn assemble(seen: &[C]) -> Self {
        let mut r = Round { r: 0, g: 0, b: 0 };
        for c in seen {
            match c {
                C::R(n) => r.r += n,
                C::G(n) => r.g += n,
                C::B(n) => r.b += n,
            }
        }
        r
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum C {
    R(usize),
    G(usize),
    B(usize),
}

peg::parser!{
    grammar game() for str {
        pub rule line() -> Game
            = "Game " id:number() ": " rounds:round() ++ "; " { Game { id, rounds } }

        rule round() -> Round
            = cs:color() ++ ", " { Round::assemble(&cs) }

        rule number() -> usize
            = n:(['0' ..= '9']+) { n.into_iter().collect::<String>().parse().unwrap() }

        rule color() -> C
            = n:number() " blue" { C::B(n) }
            / n:number() " red" { C::R(n) }
            / n:number() " green" { C::G(n) }
    }
}


fn part1_possible(g: &Game) -> usize {
    for r in &g.rounds {
        if r.r > 12 || r.g > 13 || r.b > 14 {
            return 0;
        }
    }

    g.id
}

fn part2_power(g: &Game) -> usize {
    let mut required = Round { r: 0, g: 0, b: 0 };
    for r in &g.rounds {
        required.r = max(required.r, r.r);
        required.g = max(required.g, r.g);
        required.b = max(required.b, r.b);
    }

    required.r * required.g * required.b
}

fn main() {
    let reader = BufReader::new(stdin());
    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in reader.lines() {
        let line = line.expect("read line");
        let parsed = game::line(&line).expect("parse error");

        part1_total += part1_possible(&parsed);
        part2_total += part2_power(&parsed);
    }

    println!("day2/1 sum = {part1_total}");
    println!("day2/2 sum = {part2_total}");
}
