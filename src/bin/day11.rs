use std::{fmt::{Display, Debug}, str::FromStr, collections::VecDeque};

use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day11.txt");

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    modulo: u64,
    positive_monkey_idx: usize,
    negative_monkey_idx: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey {{")?;
        writeln!(f, "  items:  {:?}", self.items)?;
        writeln!(f, "  modulo:  {}", self.modulo)?;
        writeln!(f, "  pos_idx:  {:?}", self.positive_monkey_idx)?;
        writeln!(f, "  neg_idx:  {:?}", self.negative_monkey_idx)?;
        writeln!(f, "}}")
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    // Monkey 3:
    //   Starting items: 74
    //   Operation: new = old + 3
    //   Test: divisible by 17
    //     If true: throw to monkey 0
    //     If false: throw to monkey 1
    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect::<Vec<&str>>();
        let (_, items_str) = lines[1].split_once(": ")
            .ok_or(anyhow::anyhow!("invalid input: {s}"))?;
        let items = items_str.split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
            // .map(|s| s.parse::<u64>())
            // .collect::<Result<Vec<u64>>>()?;
            //
        let (_, operation_str) = lines[2].split_once("= ")
            .ok_or(anyhow::anyhow!("invalid input: {s}"))?;
        let operation_tokens = operation_str.split(" ").collect::<Vec<&str>>();
        let x = operation_tokens[0].parse::<u64>().ok();
        let y = operation_tokens[2].parse::<u64>().ok();
        let op = operation_tokens[1].to_string();

        let operation = move |old| {
            let x = x.unwrap_or(old);
            let y = y.unwrap_or(old);
            match op.as_str() {
                "+" => x + y,
                "-" => x - y,
                "*" => x * y,
                "/" => x / y,
                _ => panic!("invalid operation: {op}"),
            }
        };
        let modulo = lines[3]
            .split_whitespace()
            .last()
            .ok_or(anyhow::anyhow!("invalid input: {s}"))?
            .parse::<u64>()?;
        let positive_monkey_idx = lines[4]
            .split_whitespace()
            .last()
            .ok_or(anyhow::anyhow!("invalid input: {s}"))?
            .parse::<usize>()?;
        let negative_monkey_idx = lines[5]
            .split_whitespace()
            .last()
            .ok_or(anyhow::anyhow!("invalid input: {s}"))?
            .parse::<usize>()?;
        Ok(Self {
            items,
            operation: Box::new(operation),
            modulo,
            positive_monkey_idx,
            negative_monkey_idx,
        })
    }
}

fn run_once(monkeys: &mut Vec<Monkey>, monkey_business: &mut Vec<u64>, soothed: bool) {
    let modulus = monkeys.iter().fold(1, |acc, m| acc * m.modulo);
    for i in 0..monkeys.len() {
        let actions = {
            let mut actions = Vec::new();
            let monkey = &mut monkeys[i];

            for item in &monkey.items {
                monkey_business[i] += 1;
                let mut worry_level = (monkey.operation)(*item);
                if soothed {
                    worry_level /= 3;
                }

                if worry_level % monkey.modulo == 0 {
                    actions.push((monkey.positive_monkey_idx, worry_level % modulus));
                } else {
                    actions.push((monkey.negative_monkey_idx, worry_level % modulus));
                }
            }

            monkey.items.clear();

            actions
        };

        for (monkey_idx, worry_level) in actions {
            monkeys[monkey_idx].items.push(worry_level);
        }
    };
}

fn part1(input: &str) -> Result<u64> {
    let mut monkeys = split_input(input, "\n\n", |s| Monkey::from_str(s))?;
    let mut monkey_business = vec![0; monkeys.len()];
    println!("monkeys: {monkeys:?}");
    println!("monkey_business: {monkey_business:?}");
    for _ in 0..20 {
        run_once(&mut monkeys, &mut monkey_business, true);
    }
    monkey_business.sort_by(|a, b| b.cmp(a));
    println!("monkeys: {monkeys:?}");
    println!("monkey_business: {monkey_business:?}");
    // let computer = build_and_run_computer(input)?;

    // let signal = computer.output
    //     .iter()
    //     .map(|(cycle, val)| *cycle as i64 * val)
    //     .sum();
    //

    Ok(monkey_business[0] * monkey_business[1])
}

fn part2(input: &str) -> Result<u64> {
    let mut monkeys = split_input(input, "\n\n", |s| Monkey::from_str(s))?;
    let mut monkey_business = vec![0; monkeys.len()];
    println!("monkeys: {monkeys:?}");
    println!("monkey_business: {monkey_business:?}");
    for _ in 0..10000 {
        run_once(&mut monkeys, &mut monkey_business, false);
    }
    monkey_business.sort_by(|a, b| b.cmp(a));
    println!("monkeys: {monkeys:?}");
    println!("monkey_business: {monkey_business:?}");
    // let computer = build_and_run_computer(input)?;

    // let signal = computer.output
    //     .iter()
    //     .map(|(cycle, val)| *cycle as i64 * val)
    //     .sum();
    //

    Ok(monkey_business[0] * monkey_business[1])
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
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
            "###
        )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        assert_eq!(10605, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(2713310158, part2(&input).unwrap());
    }
}
