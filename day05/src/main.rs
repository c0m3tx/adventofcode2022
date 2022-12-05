const INPUT: &str = include_str!("../input.txt");

mod crane;
mod parser;
mod types;

use std::error::Error;

use types::PrintableStack;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);

    Ok(())
}

fn part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let (stacks, movements) = parser::parse_input(input).unwrap();
    Ok(crane::v9000(stacks, movements).top_elements())
}

fn part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let (stacks, movements) = parser::parse_input(input)?;
    Ok(crane::v9001(stacks, movements).top_elements())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
    1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), "CMZ      ")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), "MCD      ")
    }
}
