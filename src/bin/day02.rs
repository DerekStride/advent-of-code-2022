use advent_of_code_2022::*;
use std::str::FromStr;
use anyhow::Result;


#[derive(Debug, PartialEq, Copy, Clone)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Game::Rock),
            "B" => Ok(Game::Paper),
            "C" => Ok(Game::Scissors),
            _ => Err(anyhow::anyhow!("Invalid game")),
        }
    }
}

impl Game {
    fn play(&self, other: &Game) -> Outcome {
        match (self, other) {
            (Game::Rock, Game::Paper) => Outcome::Lose,
            (Game::Rock, Game::Scissors) => Outcome::Win,
            (Game::Paper, Game::Rock) => Outcome::Win,
            (Game::Paper, Game::Scissors) => Outcome::Lose,
            (Game::Scissors, Game::Rock) => Outcome::Lose,
            (Game::Scissors, Game::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn score(&self) -> usize {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }

    fn winning_move(&self) -> Game {
        match self {
            Game::Rock => Game::Paper,
            Game::Paper => Game::Scissors,
            Game::Scissors => Game::Rock,
        }
    }

    fn losing_move(&self) -> Game {
        match self {
            Game::Rock => Game::Scissors,
            Game::Paper => Game::Rock,
            Game::Scissors => Game::Paper,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

fn run<F>(input: &str, strategy: F) -> Result<usize>
where F: Fn(&str, &Game) -> Result<Game> {
    let games = split_input(input.trim(), "\n", |s| {
        let oppenent = s[0..1].parse::<Game>()?;
        Ok((oppenent, strategy(&s[2..3], &oppenent)?))
    })?;

    let score: usize = games
        .iter()
        .map(|(a, b)| b.play(a).score() + b.score())
        .sum();
    Ok(score)
}

fn strategy1(strategy: &str, _oppenent: &Game) -> Result<Game> {
    match strategy {
        "X" => Ok(Game::Rock),
        "Y" => Ok(Game::Paper),
        "Z" => Ok(Game::Scissors),
        _ => Err(anyhow::anyhow!("Invalid strategy")),
    }
}

fn strategy2(strategy: &str, oppenent: &Game) -> Result<Game> {
    match strategy {
        "X" => Ok(oppenent.losing_move()),
        "Y" => Ok(oppenent.clone()),
        "Z" => Ok(oppenent.winning_move()),
        _ => Err(anyhow::anyhow!("Invalid strategy")),
    }
}

fn main() -> Result<()> {
    println!("Part 1: {}", run(INPUT, strategy1)?);
    println!("Part 2: {}", run(INPUT, strategy2)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = make_input(
            r###"
            A Y
            B X
            C Z
            "###
        );
        assert_eq!(15, run(&input, strategy1)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = make_input(
            r###"
            A Y
            B X
            C Z
            "###
        );
        assert_eq!(12, run(&input, strategy2)?);
        Ok(())
    }
}
