use std::collections::HashSet;
use std::error::Error;

static INPUT: &str = include_str!("../input.txt");

fn part_1() {
    let result = INPUT
        .lines()
        // split each line in half
        .map(split_compartments)
        // check which char is in common between the two
        .map(|(a, b)| find_common(&[a, b]).expect("Nothing in common?"))
        // convert the common char with priority
        .map(char_priority)
        // sum it all
        .sum::<u64>();

    println!("Part 1 sum is {result}")
}

fn part_2() {
    let result = INPUT
        .lines()
        .collect::<Vec<_>>()
        // chunk in groups of three
        .chunks(3)
        // find the common element between group
        .map(|group| find_common(group).expect("Nothing in common?"))
        // convert the common char with priority
        .map(char_priority)
        // sum it all
        .sum::<u64>();

    println!("Part 2 sum is {result}");
}

fn main() {
    part_1();
    part_2();
}

fn split_compartments(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common<'a>(compartments: &[&str]) -> Result<char, Box<dyn Error>> {
    let hashsets: Vec<HashSet<char>> = compartments
        .iter()
        .map(|comp| comp.chars().collect())
        .collect();

    let common_elements = hashsets
        .into_iter()
        .reduce(|a, b| a.intersection(&b).cloned().collect());

    common_elements
        .and_then(|hash| hash.iter().next().copied())
        .ok_or("No element found".into())
}

fn char_priority(c: char) -> u64 {
    match c {
        'a'..='z' => c as u64 - 'a' as u64 + 1,
        'A'..='Z' => c as u64 - 'A' as u64 + 27,
        _ => panic!("Should never happen"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_common {
        ($f:ident, $a:expr, $b:expr, $expected:expr) => {
            #[test]
            fn $f() {
                let result = find_common(&[$a, $b]).expect("Not found");
                assert_eq!(result, $expected)
            }
        };
    }

    #[test]
    fn test_split_compartment() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (a, b) = split_compartments(rucksack);
        assert_eq!(a, "vJrwpWtwJgWr");
        assert_eq!(b, "hcsFMMfFFhFp");
    }

    test_find_common!(common_1, "vJrwpWtwJgWr", "hcsFMMfFFhFp", 'p');
    test_find_common!(common_2, "jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL", 'L');
    test_find_common!(common_3, "PmmdzqPrV", "vPwwTWBwg", 'P');
    test_find_common!(common_4, "wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn", 'v');
    test_find_common!(common_5, "ttgJtRGJ", "QctTZtZT", 't');
    test_find_common!(common_6, "CrZsJsPPZsGz", "wwsLwLmpwMDw", 's');

    #[test]
    fn test_char_priority() {
        assert_eq!(char_priority('a'), 1);
        assert_eq!(char_priority('z'), 26);
        assert_eq!(char_priority('A'), 27);
        assert_eq!(char_priority('Z'), 52);
        assert_eq!(char_priority('p'), 16);
        assert_eq!(char_priority('L'), 38);
        assert_eq!(char_priority('P'), 42);
        assert_eq!(char_priority('v'), 22);
        assert_eq!(char_priority('t'), 20);
        assert_eq!(char_priority('s'), 19);
    }
}
