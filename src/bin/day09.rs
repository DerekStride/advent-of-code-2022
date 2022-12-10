use std::{str::FromStr, fmt::Display, collections::HashSet};
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day09.txt");

#[derive(Debug)]
enum Command {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(" ").unwrap();
        let dist = dist.parse::<usize>()?;

        let cmd = match dir {
            "R" => Self::Right(dist),
            "L" => Self::Left(dist),
            "U" => Self::Up(dist),
            "D" => Self::Down(dist),
            _ => anyhow::bail!("Invalid direction: {}", dir),
        };

        Ok(cmd)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Board {
    grid: Vec<Vec<char>>,
    knots: Vec<Point>,
    head: Point,
    tail: Point,
    tail_points: Vec<Point>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Head: {:?} | Tail: {:?}", self.head, self.tail)?;
        // for row in self.grid.iter().rev() {
        //     for col in row.iter() {
        //         write!(f, "{}  ", col)?;
        //     }
        //     writeln!(f)?;
        // }
        let mut grid = vec![vec!['.'; self.grid[0].len()]; self.grid.len()];

        for (i, knot) in self.knots.iter().enumerate().rev() {
            let ch = (i + 1) as u8 + '0' as u8;
            grid[knot.y][knot.x] = ch as char;
        }
        grid[self.head.y][self.head.x] = 'H';

        for row in grid.iter().rev() {
            for col in row.iter() {
                write!(f, "{}  ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        let mut grid = vec![vec!['.'; 1000]; 1000];
        grid[500][500] = 'H';
        Self {
            grid,
            head: Point { x: 500, y: 500 },
            knots: vec![Point { x: 500, y: 500 }; 9],
            tail: Point { x: 500, y: 500 },
            tail_points: vec![Point { x: 500, y: 500 }],
        }
    }

    fn move_right(&mut self) {
        if self.head.x >= self.grid[self.head.y].len() - 1 {
            self.expand_length();
        }
        if self.head == self.tail {
            self.grid[self.head.y][self.head.x] = 'T';
        } else {
            self.grid[self.head.y][self.head.x] = '.';
        }
        self.head.x += 1;
        self.grid[self.head.y][self.head.x] = 'H';
        self.tail_follow();
        self.knots_follow();
    }

    fn move_left(&mut self) {
        if self.head.x == 0 {
            println!("Can't move left!");
            return;
        }
        if self.head == self.tail {
            self.grid[self.head.y][self.head.x] = 'T';
        } else {
            self.grid[self.head.y][self.head.x] = '.';
        }
        self.head.x -= 1;
        self.grid[self.head.y][self.head.x] = 'H';
        self.tail_follow();
        self.knots_follow();
    }

    fn move_up(&mut self) {
        if self.head.y >= self.grid.len() - 1 {
            self.expand_height();
        }
        if self.head == self.tail {
            self.grid[self.head.y][self.head.x] = 'T';
        } else {
            self.grid[self.head.y][self.head.x] = '.';
        }
        self.head.y += 1;
        self.grid[self.head.y][self.head.x] = 'H';
        self.tail_follow();
        self.knots_follow();
    }

    fn move_down(&mut self) {
        if self.head.y == 0 {
            println!("Can't move down!");
            return;
        }
        if self.head == self.tail {
            self.grid[self.head.y][self.head.x] = 'T';
        } else {
            self.grid[self.head.y][self.head.x] = '.';
        }
        self.head.y -= 1;
        self.grid[self.head.y][self.head.x] = 'H';
        self.tail_follow();
        self.knots_follow();
    }

    fn knots_follow(&mut self) {
        self.tail_points.push(self.knots[self.knots.len() - 1].clone());
        self.knots[0] = self.tail.clone();
        for i in 1..self.knots.len() {
            let lead_x = self.knots[i - 1].x;
            let lead_y = self.knots[i - 1].y;
            let follow_x = self.knots[i].x;
            let follow_y = self.knots[i].y;
            let x_diff = lead_x as isize - follow_x as isize;
            let y_diff = lead_y as isize - follow_y as isize;


            if x_diff.abs() > 1 && y_diff.abs() > 1 {
                if x_diff > 0 {
                    self.knots[i].x += 1;
                } else {
                    self.knots[i].x -= 1;
                }
                if y_diff > 0 {
                    self.knots[i].y += 1;
                } else {
                    self.knots[i].y -= 1;
                }
            } else if x_diff.abs() > 1 {
                // self.grid[self.tail.y][self.tail.x] = '.';

                if x_diff > 0 {
                    self.knots[i].x += 1;
                } else {
                    self.knots[i].x -= 1;
                }
                self.knots[i].y = lead_y;
                // self.grid[self.tail.y][self.tail.x] = 'T';
            } else if y_diff.abs() > 1 {
                // self.grid[self.tail.y][self.tail.x] = '.';

                if y_diff > 0 {
                    self.knots[i].y += 1;
                } else {
                    self.knots[i].y -= 1;
                }
                self.knots[i].x = lead_x;
                // self.grid[self.tail.y][self.tail.x] = 'T';
            };
        }

        self.tail_points.push(self.knots[self.knots.len() - 1].clone());
    }

    // 2606
    // 2602
    // 2601
    // 2599
    // 2591
    // 2573

    fn tail_follow(&mut self) {
        let x_diff = self.head.x as isize - self.tail.x as isize;
        let y_diff = self.head.y as isize - self.tail.y as isize;
        // self.tail_points.push(self.tail);

        if x_diff.abs() > 1 {
            self.grid[self.tail.y][self.tail.x] = '.';

            if x_diff > 0 {
                self.tail.x += 1;
            } else {
                self.tail.x -= 1;
            }
            self.tail.y = self.head.y;
            self.grid[self.tail.y][self.tail.x] = 'T';
        } else if y_diff.abs() > 1 {
            self.grid[self.tail.y][self.tail.x] = '.';

            if y_diff > 0 {
                self.tail.y += 1;
            } else {
                self.tail.y -= 1;
            }
            self.tail.x = self.head.x;
            self.grid[self.tail.y][self.tail.x] = 'T';
        };
        // self.tail_points.push(self.tail);
    }

    fn expand_length(&mut self) {
        for row in self.grid.iter_mut() {
            row.push('.');
        }
    }

    fn expand_height(&mut self) {
        self.grid.push(vec!['.'; self.grid[0].len()]);
    }
}

fn part1(input: &str) -> Result<usize> {
    let commands = input
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<Command>>>()?;

    let mut board = Board::new();
    // println!("{board}");

    for command in commands {
        match command {
            Command::Right(dist) => {
                for _ in 0..dist {
                    // println!("{board}");
                    board.move_right();
                }
            },
            Command::Left(dist) => {
                for _ in 0..dist {
                    // println!("{board}");
                    board.move_left();
                }
            },
            Command::Up(dist) => {
                for _ in 0..dist {
                    // println!("{board}");
                    board.move_up();
                }
            },
            Command::Down(dist) => {
                for _ in 0..dist {
                    // println!("{board}");
                    board.move_down();
                }
            },
        };
    };

    // println!("{board}");

    let tail_points: HashSet<Point> = board.tail_points.into_iter().collect();
    // println!("{tail_points:?}");

    let mut grid = vec![vec!['.'; board.grid[0].len()]; board.grid.len()];
    grid[board.head.y][board.head.x] = 'H';

    for (i, knot) in board.knots.iter().enumerate() {
        let ch = (i + 1) as u8 + '0' as u8;
        grid[knot.y][knot.x] = ch as char;
    }

    for point in &tail_points {
        grid[point.y][point.x] = 'T';
    }

    // for row in grid.iter().rev() {
    //     for col in row.iter() {
    //         print!("{}  ", col);
    //     }
    //     println!();
    // }

    Ok(tail_points.len())
}

fn main() {
    println!("part 1: {}", part1(INPUT.trim()).unwrap());
    // println!("part 2: {}", part2(INPUT.trim()).unwrap());
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::*;
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = make_input(
            r###"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
            "###
            );

        assert_eq!(13, part1(&input.trim())?);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = make_input(
            r###"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
            "###
            );

        assert_eq!(31, part1(&input.trim())?);

        Ok(())
    }
}
