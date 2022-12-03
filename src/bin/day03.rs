use std::collections::HashSet;

use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

fn char_to_score(c: &char) -> u64 {
    match c {
        'a'..='z' => (*c as u64) - ('a' as u64) + 1,
        'A'..='Z' => (*c as u64) - ('A' as u64) + 27,
        _ => panic!("invalid char: {}", c),
    }
}

fn part1(input: &str) -> Result<u64> {
    let score = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let compartment_1: HashSet<char> = a.chars().collect();
            let compartment_2: HashSet<char> = b.chars().collect();

            char_to_score(compartment_1.intersection(&compartment_2).next().unwrap())
        })
        .sum();
    Ok(score)
}

fn part2(input: &str) -> Result<u64> {
    let score = input
        .lines()
        .collect::<Vec<&str>>()
        // Chunk into the 3 elf groups
        .chunks(3)
        .map(|slice| {
            let intersections = slice
                .iter()
                .map(|line| {
                    // Convert inventories into sets of items
                    line.chars().collect::<HashSet<char>>()
                })
                .reduce(|a, b| {
                    // Find the intersection of the 3 elf groups
                    a.intersection(&b)
                        .into_iter()
                        .map(|c| *c)
                        .collect::<HashSet<char>>()
                })
                .unwrap();

            char_to_score(intersections.iter().next().unwrap())
        })
        .sum();
    Ok(score)
}

fn main() -> Result<()> {
    println!("Part 1: {}", part1(INPUT)?);
    println!("Part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = make_input(
            r###"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "###
        );
        assert_eq!(157, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = make_input(
            r###"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "###
        );
        assert_eq!(70, part2(&input)?);
        Ok(())
    }
}
