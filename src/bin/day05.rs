const INPUT: &'static str = include_str!("../../inputs/day05.txt");

type Stack = Vec<char>;

fn print_stacks(stacks: &Vec<Stack>) {
    for (idx, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", idx + 1, stack);
    }
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let stack_lines = input.lines().rev().collect::<Vec<&str>>();
    let mut stacks = stack_lines
        .get(0)
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect::<Vec<Stack>>();

    for line in stack_lines.iter().skip(1) {
        for (stack_idx, chars) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chars[1] == ' ' {
                continue;
            };

            let stack = stacks.get_mut(stack_idx).unwrap();
            stack.push(chars[1]);
        }
    }

    stacks
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();

            Instruction {
                quantity: tokens[1].parse::<usize>().unwrap(),
                src: tokens[3].parse::<usize>().unwrap(),
                dst: tokens[5].parse::<usize>().unwrap(),
            }
        })
    .collect()
}

fn part1(input: &str) -> String {
    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks_input);
    let instructions = parse_instructions(instructions_input);
    for ins in instructions {
        for _ in 0..ins.quantity {
            let item = {
                let src_stack = stacks.get_mut(ins.src - 1).unwrap();
                src_stack.pop().unwrap()
            };
            let dst_stack = stacks.get_mut(ins.dst - 1).unwrap();
            dst_stack.push(item);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(stack.last().unwrap().clone());
    }
    result
}

fn part2(input: &str) -> String {
    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks_input);
    let instructions = parse_instructions(instructions_input);
    let mut tmp_stack = Vec::new();
    for ins in instructions {
        for _ in 0..ins.quantity {
            let item = {
                let src_stack = stacks.get_mut(ins.src - 1).unwrap();
                src_stack.pop().unwrap()
            };
            tmp_stack.push(item);
        }
        for _ in 0..ins.quantity {
            let item = tmp_stack.pop().unwrap();
            let dst_stack = stacks.get_mut(ins.dst - 1).unwrap();
            dst_stack.push(item);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(stack.last().unwrap().clone());
    }
    result
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
    [D]
[N] [C]
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
            "###
        );

        assert_eq!("CMZ", part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = make_input(
            r###"
    [D]
[N] [C]
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
            "###
        );

        assert_eq!("MCD", part2(&input));
    }
}
