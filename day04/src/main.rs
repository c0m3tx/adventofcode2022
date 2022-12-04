use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

fn overlapping(first: &RangeInclusive<u64>, second: &RangeInclusive<u64>) -> bool {
    if first.start() <= second.start() {
        first.end() >= second.start()
    } else {
        second.end() >= first.start()
    }
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

    macro_rules! assert_that {
        ($a_1:literal-$a_2:literal is superset of $b_1:literal-$b_2:literal) => {
            let a = $a_1..=$a_2;
            let b = $b_1..=$b_2;
            assert!(superset(&a, &b))
        };
        ($a_1:literal-$a_2:literal is not a superset of $b_1:literal-$b_2:literal) => {
            let a = $a_1..=$a_2;
            let b = $b_1..=$b_2;
            assert!(!superset(&a, &b))
        };
        ($a_1:literal-$a_2:literal overlaps $b_1:literal-$b_2:literal) => {
            let a = $a_1..=$a_2;
            let b = $b_1..=$b_2;
            assert!(overlapping(&a, &b))
        };
        ($a_1:literal-$a_2:literal does not overlap $b_1:literal-$b_2:literal) => {
            let a = $a_1..=$a_2;
            let b = $b_1..=$b_2;
            assert!(!overlapping(&a, &b))
        };
    }

    #[test]
    fn test_superset() {
        assert_that!(1-7 is superset of 2-5);
        assert_that!(1-4 is not a superset of 3-6);
    }

    #[test]
    fn test_overlapping() {
        assert_that!(5-7 overlaps 7-9);
        assert_that!(1-3 does not overlap 5-8);
        assert_that!(2-3 overlaps 1-4);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert_eq!(result[0], (2..=4, 6..=8));
        assert_eq!(result[3], (2..=8, 3..=7));
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
