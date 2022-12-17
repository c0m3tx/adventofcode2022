mod cavemap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    cavemap::CaveNetwork::from(input).most_exhausted_gas()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 1651);
    }
}
