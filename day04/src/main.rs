use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

fn overlapping(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.clone().any(|v| second.contains(&v))
}

fn superset(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    first.start() <= second.start() && first.end() >= second.end()
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    input
        .lines()
        .map(|line| line.split_once(",").expect("Not found"))
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        .collect()
}

fn parse_range(input: &str) -> RangeInclusive<u64> {
    input
        .split_once("-")
        .map(|(a, b)| RangeInclusive::new(a.parse().unwrap(), b.parse().unwrap()))
        .expect("Invalid input")
}

fn main() {
    println!("Subsets: {}", part_1(INPUT));
    println!("Overlaps: {}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(a, b)| superset(a, b) || superset(b, a))
        .count()
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(a, b)| overlapping(a, b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result[0], (2..=4, 6..=8));
        assert_eq!(result[3], (2..=8, 3..=7));
    }

    #[test]
    fn test_superset() {
        let range_1 = 1..=7;
        let range_2 = 2..=5;
        assert!(superset(&range_1, &range_2));
    }

    #[test]
    fn test_overlapping() {
        let range_1 = 5..=7;
        let range_2 = 7..=9;
        assert!(overlapping(&range_1, &range_2))
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 4);
    }
}
