use std::str::FromStr;
use anyhow::Result;

use advent_of_code_2022::*;

const INPUT: &'static str = include_str!("../../inputs/day10.txt");

fn main() {
    println!("part 1: {}", part1(INPUT).unwrap());
    println!("part 2:\n{}", part2(INPUT).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Command {
    Noop,
    Add(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.trim() == "noop" {
            return Ok(Command::Noop);
        }
        let (_, val) = s.split_once(" ")
            .ok_or(anyhow::anyhow!("invalid command: {s}"))?;

        Ok(Command::Add(val.parse::<i64>()?))

    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Computer {
    reg_x: i64,
    cycle: u64,
    output: Vec<(u64, i64)>,
    screen: Vec<Vec<char>>,
}

impl Computer {
    fn new() -> Self {
        Self {
            reg_x: 1,
            cycle: 0,
            output: vec![],
            screen: vec![vec![' '; 40]; 6],
        }
    }

    fn run(&mut self, cmd: &Command) {
        match cmd {
            Command::Noop => {
                self.cycle += 1;
                self.draw();
                self.check_output();
            },
            Command::Add(val) => {
                self.cycle += 1;
                self.draw();
                self.check_output();
                self.cycle += 1;
                self.draw();
                self.check_output();
                self.reg_x += val;
            }
        }
    }

    fn check_output(&mut self) {
        if self.cycle == 20 {
            self.output.push((self.cycle, self.reg_x));
        } else if self.cycle > 20 && (self.cycle - 20) % 40 == 0 {
            self.output.push((self.cycle, self.reg_x));
        };
    }

    fn draw(&mut self) {
        let pos = (self.cycle - 1) % 240;
        let x = (pos / 40) as usize;
        let y = (pos % 40) as usize;

        if self.reg_x < 0 {
            println!("{}: Can't handle negative x values: {}", self.cycle, self.reg_x);
            // return;
        };

        if (y as i64) < (self.reg_x - 1) {
            self.screen[x][y] = '.';
            return;
        } else if (y as i64) > (self.reg_x + 1) {
            self.screen[x][y] = '.';
            return;
        }

        self.screen[x][y] = '#';
    }

    fn screen(&self) -> String {
        self.screen
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn build_and_run_computer(input: &str) -> Result<Computer> {
    let mut computer = Computer::new();
    for line in input.trim().lines() {
        let cmd = line.parse::<Command>()?;
        computer.run(&cmd);
    }
    Ok(computer)
}

fn part1(input: &str) -> Result<i64> {
    let computer = build_and_run_computer(input)?;

    let signal = computer.output
        .iter()
        .map(|(cycle, val)| *cycle as i64 * val)
        .sum();

    Ok(signal)
}

fn part2(input: &str) -> Result<String> {
    let computer = build_and_run_computer(input)?;
    // println!();
    // println!("{}", computer.screen());
    // println!();

    Ok(computer.screen())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> String {
        make_input(
            r###"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
            "###
            )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        let computer = build_and_run_computer(&input).unwrap();
        let expected = vec![
            (20, 21),
            (60, 19),
            (100, 18),
            (140, 21),
            (180, 16),
            (220, 18),
        ];
        assert_eq!(expected, computer.output);
        assert_eq!(13140, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        let expected = r###"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
        "###;
        let actual = part2(&input).unwrap();
        println!("expected:");
        println!("{expected}");
        println!("actual:");
        println!("{actual}");
        assert_eq!(expected.trim(), actual.trim());
    }
}
