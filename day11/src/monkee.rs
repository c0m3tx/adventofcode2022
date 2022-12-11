use crate::operation::Operation;
use std::error::Error;

pub struct Monkee {
    items: Vec<isize>,
    operation: Operation,
    pub test: isize,
    if_true: usize,
    if_false: usize,
    pub inspected_items: usize,
}

pub fn do_a_turn<F>(monkees: &mut Vec<Monkee>, worry_decrease_fn: F)
where
    F: Fn(isize) -> isize,
{
    for i in 0..monkees.len() {
        let current_monkee = &mut monkees[i];
        let mut swaps: Vec<(usize, isize)> = Vec::new();
        current_monkee.items.iter().for_each(|item| {
            current_monkee.inspected_items += 1;
            let new_value = current_monkee.operation.apply(*item);
            let new_value = worry_decrease_fn(new_value);
            if new_value % current_monkee.test == 0 {
                swaps.push((current_monkee.if_true, new_value));
            } else {
                swaps.push((current_monkee.if_false, new_value));
            }
        });

        current_monkee.items = vec![];
        for (monkee, value) in swaps {
            monkees[monkee].items.push(value);
        }
    }
}

impl TryFrom<&[&str]> for Monkee {
    type Error = Box<dyn Error>;
    fn try_from(data: &[&str]) -> Result<Self, Self::Error> {
        let items = data[1]
            .trim_start_matches("  Starting items: ")
            .split(", ")
            .map(|item| item.parse().map_err(|_| "Invalid number"))
            .collect::<Result<_, _>>()?;
        let operation: Operation = data[2].trim_start_matches("  Operation: new = ").into();
        let test: isize = data[3]
            .trim_start_matches("  Test: divisible by ")
            .parse()?;
        let if_true = data[4]
            .trim_start_matches("    If true: throw to monkey ")
            .parse()?;
        let if_false = data[5]
            .trim_start_matches("    If false: throw to monkey ")
            .parse()?;
        Ok(Monkee {
            items,
            operation,
            test,
            if_false,
            if_true,
            inspected_items: 0,
        })
    }
}

pub fn parse_all(input: &str) -> Result<Vec<Monkee>, Box<dyn Error>> {
    let lines: Vec<_> = input.lines().collect();
    lines
        .chunks(7)
        .map(|chunk| chunk.try_into())
        .collect::<Result<Vec<Monkee>, Box<dyn Error>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation::{Item, Operation};

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_parse_monkeeee() {
        let rows: Vec<&str> = TEST_INPUT.lines().take(7).collect();
        let monkee: Monkee = rows.as_slice().try_into().expect("Invalid data");
        assert_eq!(monkee.items, vec![79, 98]);
        assert_eq!(
            monkee.operation,
            Operation::Mult(Item::Current, Item::Constant(19))
        );
        assert_eq!(monkee.test, 23);
        assert_eq!(monkee.if_true, 2);
        assert_eq!(monkee.if_false, 3);
    }

    #[test]
    fn test_parse_all_monkeys() {
        let monkeys = parse_all(TEST_INPUT).unwrap();
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_do_a_turn() {
        let mut monkeys = parse_all(TEST_INPUT).unwrap();
        do_a_turn(&mut monkeys, |f| f / 3);

        assert_eq!(monkeys[0].inspected_items, 2);
        assert_eq!(monkeys[1].inspected_items, 4);
        assert_eq!(monkeys[2].inspected_items, 3);
        assert_eq!(monkeys[3].inspected_items, 5);

        assert_eq!(monkeys[0].items.len(), 4);
        assert_eq!(monkeys[1].items.len(), 6);
        assert_eq!(monkeys[2].items.len(), 0);
        assert_eq!(monkeys[3].items.len(), 0);
    }
}
