use std::{str::FromStr, fmt::Display};

use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day14.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    start: Point,
    end: Point
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(",")
            .ok_or(anyhow::anyhow!("invalid point: {s}"))?;
        Ok(Self { x: x.parse()?, y: y.parse()? })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    grid: Vec<Vec<char>>,
    width: (i64, i64),
    height: (i64, i64),
    sandiness: i64,
}

impl Grid {
    fn new(lines: Vec<Line>) -> Result<Self> {
        let width = lines
            .iter()
            .map(|line| {
                (
                    std::cmp::min(line.start.x, line.end.x),
                    std::cmp::max(line.start.x, line.end.x)
                )
            })
            .fold((i64::MAX, i64::MIN), |acc, (x1, x2)| {
                (
                    std::cmp::min(acc.0, x1),
                    std::cmp::max(acc.1, x2)
                )
            });

        let height = lines
            .iter()
            .map(|line| std::cmp::max(line.start.y, line.end.y))
            .max().ok_or(anyhow::anyhow!("no lines"))?;

        let mut grid = vec![vec!['.'; (width.1 - width.0 + 1) as usize]; (height + 1) as usize];

        for line in lines {
            if line.start.x == line.end.x {
                let start = std::cmp::min(line.start.y, line.end.y);
                let end = std::cmp::max(line.start.y, line.end.y);
                let width = (line.start.x - width.0) as usize;

                for y in start..=end {
                    grid[y as usize][width] = '#';
                }
            } else {
                let start = std::cmp::min(line.start.x, line.end.x);
                let end = std::cmp::max(line.start.x, line.end.x);

                for x in start..=end {
                    let x = (x - width.0) as usize;
                    grid[line.start.y as usize][x] = '#';
                }
            }

        }

        Ok(Self {
            grid,
            width,
            height: (0, height + 1),
            sandiness: 0,
        })
    }

    fn drop_sand(&mut self) -> bool {
        self.sandiness += 1;
        let mut point = Point { x: 500 - self.width.0, y: 0 };

        loop {
            match self.move_sand(point) {
                Some(Some(new_point)) => point = new_point,
                Some(None) => return true,
                None => return false,
            }
        }
    }

    fn move_sand(&mut self, sand: Point) -> Option<Option<Point>> {
        if sand.y == self.height.1 {
            return None;
        }
        let row = sand.y as usize;
        let column = sand.x as usize;

        if row + 1 == self.grid.len() {
            return None;
        }

        if self.grid[row + 1][column] == '.' {
            self.grid[row][column] = '.';
            self.grid[row + 1][column] = 'o';
            return Some(Some(Point { x: sand.x, y: sand.y + 1 }));
        };

        if column == 0 {
            return None;
        }


        if self.grid[row + 1][column - 1] == '.' {
            self.grid[row][column] = '.';
            self.grid[row + 1][column - 1] = 'o';
            return Some(Some(Point { x: sand.x - 1, y: sand.y + 1 }));
        }

        if column + 1 == self.grid[row].len() {
            return None;
        }

        if self.grid[row + 1][column + 1] == '.' {
            self.grid[row][column] = '.';
            self.grid[row + 1][column + 1] = 'o';
            return Some(Some(Point { x: sand.x + 1, y: sand.y + 1 }));
        }

        if sand.x == 500 - self.width.0 && sand.y == 0 {
            return None;
        }

        Some(None)
    }

    fn expand(&mut self) {
        self.height.1 += 2;

        for _ in 0..1500 {
            for row in self.grid.iter_mut() {
                row.insert(0, '.');
                row.push('.');
            }
            self.width.0 -= 1;
            self.width.1 += 1;
        }

        self.grid.push(vec!['.'; self.grid[0].len()]);
        self.grid.push(vec!['#'; self.grid[0].len()]);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "w: {:?}, h: {:?}, sand: {}", self.width, self.height, self.sandiness)?;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if y == 0 && x as i64 + self.width.0 == 500 {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<i64> {
    let point_groups = input
        .lines()
        .map(|line| {
            line
                .trim()
                .split(" -> ")
                .filter_map(|point| point.parse::<Point>().ok())
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut lines = Vec::new();

    for points in point_groups {
        if points.is_empty() { continue; }
        let mut start = points[0];

        for point in points[1..].iter() {
            lines.push(Line { start, end: *point });
            start = *point;
        }
    }

    let mut grid = Grid::new(lines.clone())?;
    let mut c = 0;
    loop {
        if !grid.drop_sand() { break; }
    }
    println!("{}", grid);
    Ok(grid.sandiness - 1)
}

fn part2(input: &str) -> Result<i64> {
    let point_groups = input
        .lines()
        .map(|line| {
            line
                .trim()
                .split(" -> ")
                .filter_map(|point| point.parse::<Point>().ok())
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut lines = Vec::new();

    for points in point_groups {
        if points.is_empty() { continue; }
        let mut start = points[0];

        for point in points[1..].iter() {
            lines.push(Line { start, end: *point });
            start = *point;
        }
    }

    let mut grid = Grid::new(lines.clone())?;
    println!("{}", grid);
    grid.expand();
    println!("{}", grid);
    loop {
        if !grid.drop_sand() { break; }
    }
    println!("{}", grid);
    Ok(grid.sandiness)
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
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
            "###
            )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        assert_eq!(24, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(93, part2(&input).unwrap());
    }
}
