use std::str::FromStr;



const INPUTS: &'static str = include_str!("../../inputs/day00.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Input {
    Dir(Direction),
    Pos(Point),
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, data) = s.split_once(" ").unwrap();
        match label {
            "Direction" => Ok(Input::Dir(Direction::from_str(data)?)),
            "Point" => Ok(Input::Pos(Point::from_str(data)?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Up" => Ok(Direction::Up),
            "Down" => Ok(Direction::Down),
            "Left" => Ok(Direction::Left),
            "Right" => Ok(Direction::Right),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s[1..(s.len() - 1)].split_once(", ").unwrap();
        Ok(Point(x.parse::<i32>()?, y.parse::<i32>()?))
    }
}

fn main() {
    let result = INPUTS
        .lines()
        .map(|line| line.parse::<Input>().unwrap())
        .collect::<Vec<Input>>();

    println!("Day 1:\n{:?}", result);
}
