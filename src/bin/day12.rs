use std::{str::FromStr, fmt::Display};

use advent_of_code_2022::*;
use anyhow::Result;
use pathfinding::prelude::dijkstra;

const INPUT: &'static str = include_str!("../../inputs/day12.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn adjust(&self, position: &(usize, usize)) -> Option<(usize, usize)> {
        let mut result = position.clone();
        match self {
            Move::Up => {
                if result.0 == 0 {
                    return None;
                }
                result.0 -= 1;
            },
            Move::Down => result.0 += 1,
            Move::Left => {
                if result.1 == 0 {
                    return None;
                }
                result.1 -= 1;
            },
            Move::Right => result.1 += 1,
        };
        Some(result)
    }

    fn to_char(&self) -> char {
        match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    grid: Vec<Vec<char>>,
    position: (usize, usize),
    goal: (usize, usize),
    start: (usize, usize),
    visited: Vec<((usize, usize), Move)>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = s.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut position = (0, 0);
        let mut start = (0, 0);
        let mut goal = (0, 0);

        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == 'S' {
                    start = (x, y);
                    position = (x, y);
                } else if *cell == 'E' {
                    goal = (x, y);
                }
            }
        }

        Ok(Self { grid, position, start, goal, visited: vec![(position.clone(), Move::Left)] })
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = self.grid.clone();

        for (pos, m) in self.visited.iter() {
            grid[pos.0][pos.1] = m.to_char();
        }

        writeln!(f, "Position: {:?} | Goal: {:?} | Visited: {}", self.position, self.goal, self.visited.len())?;

        grid[self.start.0][self.start.1] = 'S';
        grid[self.goal.0][self.goal.1] = 'E';
        grid[self.position.0][self.position.1] = '#';

        for row in grid.iter() {
            for col in row.iter() {
                write!(f, "{col}  ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Game {
    fn calculate_elevation(&self, position: &(usize, usize)) -> isize {
        match self.grid[position.0][position.1] {
            x @ 'a'..='z' => x as isize,
            'S' => 'a' as isize,
            'E' => 'z' as isize,
            _ => panic!("Invalid cell: {}", self.grid[position.0][position.1]),
        }
    }

    fn available_moves(&self, current_position: &(usize, usize)) -> Vec<((usize, usize), usize)> {
        let mut moves = vec![];
        let current_elevation = self.calculate_elevation(current_position);

        for m in vec![Move::Up, Move::Down, Move::Left, Move::Right] {
            let position = if let Some(pos) = m.adjust(current_position) {
                pos
            } else {
                continue;
            };

            if self.grid.len() <= position.0 || self.grid[position.0].len() <= position.1 { continue; };
            if self.visited.iter().any(|(pos, _)| *pos == position) { continue; };

            let elevation = self.calculate_elevation(&position);

            if elevation - current_elevation > 1 { continue; };

            moves.push((position, 1));
        }

        moves
    }
}

fn part1(input: &str) -> Result<usize> {
    let game = Game::from_str(input.trim())?;
    let result = dijkstra(&game.start, |position| game.available_moves(position), |position| position == &game.goal)
        .ok_or(anyhow::anyhow!("No solution found"))?;
    Ok(result.1)
}

fn part2(input: &str) -> Result<usize> {
    let game = Game::from_str(input.trim())?;
    let mut starting_positions = vec![game.start];

    for (x, row) in game.grid.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'a' {
                starting_positions.push((x, y));
            }
        }
    }

    let mut min = usize::MAX;
    for pos in starting_positions.iter() {
        if let Some(result) = dijkstra(pos, |position| game.available_moves(position), |position| position == &game.goal) {
            if result.1 < min {
                min = result.1
            }
        }
    }

    Ok(min)
}

fn main() {
    println!("part 1: {}", part1(INPUT).unwrap());
    println!("part 2: {}", part2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        make_input(
            r###"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
            "###
            )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        assert_eq!(31, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(29, part2(&input).unwrap());
    }
}
