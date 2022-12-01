use advent_of_code_2022::*;

use anyhow::Result;

const INPUTS: &'static str = include_str!("../../inputs/day01.txt");

fn elf_inventory() -> Result<Vec<usize>> {
    let inv = INPUTS
        .lines()
        .fold(vec![0], |mut acc, line| {
            if line == "" {
                acc.push(0);
            } else {
                *acc.last_mut().unwrap() += line.parse::<usize>().unwrap();
            };
            acc
        });
    Ok(inv)
}

fn main() {
    let inv = elf_inventory().unwrap();
    let mut sorted_inv = inv
        .iter()
        .enumerate()
        .map(|(i, total)| (i + 1, *total))
        .collect::<Vec<(usize, usize)>>();
    sorted_inv.sort_by(|(_, a), (_, b)| b.cmp(a));

    let total_snacks: usize = sorted_inv[..3]
        .iter()
        .map(|(_, total)| total)
        .sum();
    println!("Day 1: {:?}", total_snacks);
}
