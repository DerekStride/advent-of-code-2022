use std::ops::{RangeInclusive, RangeBounds};

use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day04.txt");

fn covers<T: PartialOrd>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> bool {
    range1.start() <= range2.start() && range1.end() >= range2.end()
}

fn range_pairs(input: &str) -> Vec<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            let (range1_start, range1_end) = a.split_once("-").unwrap();
            let (range2_start, range2_end) = b.split_once("-").unwrap();
            // (a, b)
            let range1 = RangeInclusive::new(range1_start.parse::<i64>().unwrap(), range1_end.parse::<i64>().unwrap());
            let range2 = RangeInclusive::new(range2_start.parse::<i64>().unwrap(), range2_end.parse::<i64>().unwrap());
            (range1, range2)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let range_pairs = range_pairs(input);
    let overlapping_ranges = range_pairs
        .iter()
        .filter(|(range1, range2)| {
            covers(range1, range2) || covers(range2, range1)
        });
    overlapping_ranges.count()
}

fn part2(input: &str) -> usize {
    let range_pairs = range_pairs(input);
    let overlapping_ranges = range_pairs
        .iter()
        .filter(|(range1, range2)| {
            range1.contains(range2.start()) ||
                range1.contains(range2.end()) ||
                range2.contains(range1.start()) ||
                range2.contains(range1.end())
        });
    overlapping_ranges.count()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::*;
    use super::*;

    #[test]
    fn test_part1() {
        let input = make_input(
            r###"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
            "###
        );
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = make_input(
            r###"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
            "###
        );
        assert_eq!(4, part2(&input));
    }
}
