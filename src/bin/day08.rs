use std::str::Chars;

use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day08.txt");

fn transpose(input: &Vec<String>) -> Vec<String> {
    let mut transpose = Vec::new();
    for i in 0..input[0].len() {
        let mut line = String::new();
        for j in 0..input.len() {
            line.push(input[j].chars().nth(i).unwrap());
        }
        transpose.push(line);
    }
    transpose
}

fn is_visible(row: &str, pos: usize) -> bool {
    let height = row.get(pos..=pos).unwrap().parse::<usize>().unwrap();

    for i in 0..pos {
        if row.get(i..=i).unwrap().parse::<usize>().unwrap() >= height {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> usize {
    let horizontal = input
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    let vertical = transpose(&horizontal);
    let hmax = horizontal[0].len();
    let vmax = vertical[0].len();

    let mut invisible = 0;

    for x in 1..(horizontal.len() - 1) {
        let left_to_right = &horizontal[x];
        let right_to_left = left_to_right.chars().rev().collect::<String>();

        for y in 1..(vertical.len() - 1) {
            let top_to_bottom = &vertical[y];
            let bottom_to_top = top_to_bottom.chars().rev().collect::<String>();

            if is_visible(&left_to_right, y) { continue; }
            if is_visible(&right_to_left, vmax - y - 1) { continue; }
            if is_visible(&top_to_bottom, x) { continue; }
            if is_visible(&bottom_to_top, hmax - x - 1) { continue; }

            invisible += 1;
        }
    }

    hmax * vmax - invisible
}

fn scenic_score(row: &str, pos: usize) -> u64 {
    let spot_height = row.get(pos..=pos).unwrap().parse::<usize>().unwrap();

    let mut score = 0;

    for i in (0..pos).rev() {
        let height = row.get(i..=i).unwrap().parse::<usize>().unwrap();

        if height >= spot_height {
            return score + 1;
        }
        score += 1;
    }

    score
}

fn part2(input: &str) -> u64 {
    let horizontal = input
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    let vertical = transpose(&horizontal);
    let hmax = horizontal[0].len();
    let vmax = vertical[0].len();

    let mut score = 0;

    for x in 1..(horizontal.len() - 1) {
        let left_to_right = &horizontal[x];
        let right_to_left = left_to_right.chars().rev().collect::<String>();

        for y in 1..(vertical.len() - 1) {
            let top_to_bottom = &vertical[y];
            let bottom_to_top = top_to_bottom.chars().rev().collect::<String>();

            let mut spot_score = 1;
            spot_score *= scenic_score(&left_to_right, y);
            spot_score *= scenic_score(&right_to_left, vmax - y - 1);
            spot_score *= scenic_score(&top_to_bottom, x);
            spot_score *= scenic_score(&bottom_to_top, hmax - x - 1);

            if score < spot_score {
                score = spot_score;
            }
        }
    }

    score
}

fn main() {
    println!("part 1: {}", part1(INPUT.trim()));
    println!("part 2: {}", part2(INPUT.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = make_input(
            r###"
30373
25512
65332
33549
35390
            "###
        );

        assert_eq!(21, part1(&input.trim()));
    }

    #[test]
    fn test_part2() {
        let input = make_input(
            r###"
30373
25512
65332
33549
35390
            "###
        );

        assert_eq!(2, scenic_score("33549", 2));
        assert_eq!(2, scenic_score("94533", 2));
        assert_eq!(2, scenic_score("35353", 3));
        assert_eq!(1, scenic_score("35353", 1));

        assert_eq!(8, part2(&input.trim()));
    }
}
