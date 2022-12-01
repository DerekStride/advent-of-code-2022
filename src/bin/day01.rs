use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day01.txt");

fn main() -> Result<()> {
    let mut elf_inventory = split_input(INPUT.trim(), "\n\n", |s| {
        let r = split_input(s, "\n", |s| Ok(s.parse::<usize>()?))?;
        Ok(r.iter().sum::<usize>())
    })?;

    elf_inventory.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {:?}", elf_inventory[0]);

    let total_snacks: usize = elf_inventory[..3].iter().sum();
    println!("Part 2: {:?}", total_snacks);

    Ok(())
}
