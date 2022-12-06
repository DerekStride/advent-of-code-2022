use std::collections::HashSet;


const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn chars_till_distinct(input: &str, distinct: usize) -> i32 {
    let mut buffer = vec!['\0'; distinct];

    for (i, ch) in input.chars().enumerate() {
        buffer[i % distinct] = ch;
        if i < distinct {
            continue;
        }

        let set: HashSet<&char> = HashSet::from_iter(buffer.iter());

        if set.len() == distinct {
            return 1 + i as i32;
        }
    }

    -1
}

fn main() {
    println!("part 1: {}", chars_till_distinct(INPUT, 4));
    println!("part 2: {}", chars_till_distinct(INPUT, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(7, chars_till_distinct("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, chars_till_distinct("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, chars_till_distinct("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, chars_till_distinct("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, chars_till_distinct("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, chars_till_distinct("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, chars_till_distinct("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, chars_till_distinct("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, chars_till_distinct("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, chars_till_distinct("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
