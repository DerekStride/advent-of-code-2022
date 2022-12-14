use advent_of_code_2022::*;
use anyhow::Result;

const INPUT: &'static str = include_str!("../../inputs/day14.txt");

fn part1(input: &str) -> Result<isize> {
    Ok(0)
}

fn part2(input: &str) -> Result<isize> {
    Ok(0)
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
        assert_eq!(13, part1(&input).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = example_input();
        assert_eq!(140, part2(&input).unwrap());
    }
}
