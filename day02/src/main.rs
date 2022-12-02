trait Parser {
    fn parse(&self, input: &str) -> (u8, u8);
}

// Rock 0
// Paper 1
// Scissor 2

struct BaseParser;
struct ExtendedParser;

impl Parser for BaseParser {
    fn parse(&self, input: &str) -> (u8, u8) {
        let first_value = input.chars().nth(0).unwrap() as u8 - 'A' as u8;
        let second_value = input.chars().nth(2).unwrap() as u8 - 'X' as u8;
        (first_value, second_value)
    }
}

impl Parser for ExtendedParser {
    fn parse(&self, input: &str) -> (u8, u8) {
        let first_value = input.chars().nth(0).unwrap() as u8 - 'A' as u8;
        let second_value = input.chars().nth(2).unwrap();

        match second_value {
            'X' => (first_value, (first_value + 2) % 3),
            'Y' => (first_value, first_value),
            'Z' => (first_value, (first_value + 4) % 3),
            _ => panic!("Unexpected value"),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", calculate(input, ExtendedParser));
}

fn outcome(p1: u8, p2: u8) -> u64 {
    if (p1 + 1) % 3 == p2 {
        // win
        7 + p2 as u64
    } else if p1 == p2 {
        // even
        4 + p2 as u64
    } else {
        // loss
        1 + p2 as u64
    }
}

fn calculate(input: &str, parser: impl Parser) -> u64 {
    input
        .lines()
        .map(|l| parser.parse(l))
        .map(|(a, b)| outcome(a, b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcome() {
        assert_eq!(outcome(0, 1), 8);
        assert_eq!(outcome(1, 0), 1);
        assert_eq!(outcome(2, 2), 6);
    }

    #[test]
    fn test_base_parser() {
        assert_eq!(BaseParser.parse("A X"), (0, 0));
        assert_eq!(BaseParser.parse("B Y"), (1, 1));
        assert_eq!(BaseParser.parse("C Z"), (2, 2));
    }

    #[test]
    fn test_extended_parser() {
        assert_eq!(ExtendedParser.parse("A X"), (0, 2));
        assert_eq!(ExtendedParser.parse("B X"), (1, 0));
        assert_eq!(ExtendedParser.parse("C X"), (2, 1));

        assert_eq!(ExtendedParser.parse("A Y"), (0, 0));
        assert_eq!(ExtendedParser.parse("B Y"), (1, 1));
        assert_eq!(ExtendedParser.parse("C Y"), (2, 2));

        assert_eq!(ExtendedParser.parse("A Z"), (0, 1));
        assert_eq!(ExtendedParser.parse("B Z"), (1, 2));
        assert_eq!(ExtendedParser.parse("C Z"), (2, 0));
    }

    #[test]
    fn test_parse_outcome() {
        let (a, b) = ExtendedParser.parse("A Y");
        assert_eq!(outcome(a, b), 4);

        let (a, b) = ExtendedParser.parse("B X");
        assert_eq!(outcome(a, b), 1);

        let (a, b) = ExtendedParser.parse("C Z");
        assert_eq!(outcome(a, b), 7)
    }

    #[test]
    fn test_input_base() {
        assert_eq!(calculate("A Y\nB X\nC Z\n", BaseParser), 15)
    }

    #[test]
    fn test_input_extended() {
        assert_eq!(calculate("A Y\nB X\nC Z\n", ExtendedParser), 12)
    }
}
