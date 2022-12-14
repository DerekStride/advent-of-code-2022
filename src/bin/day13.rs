use std::{str::FromStr, fmt::Display, cmp::Ordering};

use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day13.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(isize),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(list) => {
                write!(f, "[")?;
                write!(f, "{}", list.iter().map(|p| format!("{p}")).collect::<Vec<String>>().join(","))?;
                write!(f, "]")
            }
            Packet::Number(number) => write!(f, "{}", number),
        }
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "[]" {
            return Ok(Packet::List(Vec::new()));
        } else if !s.starts_with('[') {
            return Ok(Packet::Number(s.parse()?));
        };

        let mut stack: Vec<(Vec<Packet>, usize)> = Vec::new();
        for (idx, ch) in s.chars().enumerate() {
            if ch == '[' {
                stack.push((Vec::new(), idx + 1));
            } else if ch == ']' {
                let (mut packets, start_idx) = stack.pop().ok_or(anyhow::anyhow!("No start a"))?;
                if start_idx == idx && stack.is_empty() {
                    stack.push((packets, start_idx + 1));
                } else if start_idx == idx {
                    let (mut parent, _) = stack.pop().ok_or(anyhow::anyhow!("No start b"))?;
                    parent.push(Packet::List(packets));
                    stack.push((parent, idx + 1));
                } else {
                    let packet = s[start_idx..idx].parse::<Packet>()?;
                    packets.push(packet);
                    stack.push((packets, idx + 1));
                }
            } else if ch == ',' {
                let (mut packets, start_idx) = stack.pop().ok_or(anyhow::anyhow!("No start c"))?;
                if start_idx == idx && stack.is_empty() {
                    stack.push((packets, start_idx + 1));
                } else if start_idx == idx {
                    let (mut parent, _) = stack.pop().ok_or(anyhow::anyhow!("No start d"))?;
                    parent.push(Packet::List(packets));
                    stack.push((parent, idx + 1));
                } else {
                    let packet = s[start_idx..idx].parse::<Packet>()?;
                    packets.push(packet);
                    stack.push((packets, idx + 1));
                }
            };
        }

        let (packets, _) = stack.pop().ok_or(anyhow::anyhow!("Nothing left on the stack."))?;

        Ok(Packet::List(packets))
    }
}

fn compare_packets(left: &Packet, right: &Packet) -> Option<bool> {
    // println!("Compare {left} vs {right}");

    match left {
        Packet::Number(l) => {
            match right {
                Packet::Number(r) => {
                    if l < r { return Some(true); }
                    if l > r { return Some(false); }
                    None
                },
                Packet::List(r) => {
                    compare_packets(&Packet::List(vec![Packet::Number(*l)]), right)
                },
            }
        },
        Packet::List(l) => {
            match right {
                Packet::Number(r) => {
                    compare_packets(left, &Packet::List(vec![Packet::Number(*r)]))
                },
                Packet::List(r) => {
                    for (idx, p) in l.iter().enumerate() {
                        if idx >= r.len() { return Some(false); };
                        if let Some(result) = compare_packets(p, &r[idx]) {
                            // if result { return Some(true); }
                            return Some(result);
                        };
                    }
                    if l.len() < r.len() {
                        Some(true)
                    } else {
                        None
                    }
                },
            }
        },
    }
}

fn part1(input: &str) -> Result<usize> {
    let pairs = split_input(input.trim(), "\n\n", |lines| {
        let (left, right) = lines.split_once("\n")
            .ok_or(anyhow::anyhow!("invalid input: {lines}"))?;
        Ok((left.parse::<Packet>()?, right.parse::<Packet>()?))
    })?;

    let mut sum = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if let Some(result) = compare_packets(&pair.0, &pair.1) {
            if result {
                sum += idx + 1;
            }
        } else {
            panic!("No result for {pair:?}");
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut packets = input.trim().lines()
        .filter_map(|line| line.parse::<Packet>().ok())
        .collect::<Vec<Packet>>();

    let divider_1 = Packet::from_str("[[6]]")?;
    let divider_2 = Packet::from_str("[[2]]")?;
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_by(|a, b| {
        if let Some(result) = compare_packets(a, b) {
            if result {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            panic!("No result for {a:?} vs {b:?}");
        }
    });

    let mut result = 1;
    for (idx, packet) in packets.iter().enumerate() {
        if divider_1 == *packet  || divider_2 == *packet {
            result *= idx + 1;
        }
    }
    Ok(result)
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
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
            "###
            )
    }

    #[test]
    fn test_part1() {
        let input = example_input();
        assert_eq!(13, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(140, part2(&input).unwrap());
    }
}
