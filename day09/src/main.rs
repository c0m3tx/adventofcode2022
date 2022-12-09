#![allow(dead_code)]

use std::collections::HashSet;

mod command;
mod point;
mod ropes;

use command::Command;

use ropes::*;

const INPUT: &str = include_str!("../input.txt");

fn common_part(mut rope: impl Rope, commands: Vec<Command>) -> usize {
    commands
        .into_iter()
        .flat_map(|command| rope.execute_command(&command).into_iter())
        .collect::<HashSet<_>>()
        .len()
}

fn part_1(input: &str) -> usize {
    let rope = SimpleRope::default();
    let commands = command::parse_commands(input);
    common_part(rope, commands)
}

fn part_2(input: &str) -> usize {
    let rope = ComplexRope::default();
    let commands = command::parse_commands(input);
    common_part(rope, commands)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const PART_2_TEST_INPUT: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
    #[test]
    fn test_part_1() {
        let result = part_1(PART_1_TEST_INPUT);
        assert_eq!(result, 13)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(PART_2_TEST_INPUT);
        assert_eq!(result, 36)
    }
}
