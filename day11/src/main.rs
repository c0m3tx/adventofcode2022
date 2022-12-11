const INPUT: &str = include_str!("../input.txt");

mod monkee;
mod operation;

fn part_1(input: &str) -> usize {
    let mut monkees = monkee::parse_all(input).expect("Error parsing monkeys");
    for _ in 0..20 {
        monkee::do_a_turn(&mut monkees, |v| v / 3);
    }

    let mut inspected_items: Vec<_> = monkees.iter().map(|m| m.inspected_items).collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    inspected_items[0] * inspected_items[1]
}

fn part_2(input: &str) -> usize {
    let mut monkees = monkee::parse_all(input).expect("Error parsing monkeys");
    let worry_divisor: isize = monkees
        .iter()
        .map(|m| m.test)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();

    for _ in 0..10000 {
        monkee::do_a_turn(&mut monkees, |f| f % worry_divisor);
    }

    let mut inspected_items: Vec<_> = monkees.iter().map(|m| m.inspected_items).collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    inspected_items[0] * inspected_items[1]
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 2713310158);
    }
}
