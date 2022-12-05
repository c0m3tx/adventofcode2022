use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use std::error::Error;

#[derive(Parser)]
#[grammar = "input.pest"] // relative to src
struct InputParser;

use crate::types::MoveCommand;
use crate::types::Stacks;

pub fn parse_input(input: &str) -> Result<(Stacks, Vec<MoveCommand>), Box<dyn Error>> {
    let mut move_commands = vec![];
    let mut initial_status_rows = vec![];

    for line in input.lines() {
        let rule = InputParser::parse(Rule::row, line);
        if rule.is_err() {
            continue;
        }

        let rule = rule?.next().ok_or("Invalid line")?;

        match rule.as_rule() {
            Rule::initialStatus => {
                initial_status_rows.push(parse_initial_status(rule.into_inner()))
            }

            Rule::moveCommand => move_commands.push(parse_move_command(rule.into_inner())?),
            _ => unreachable!(),
        }
    }

    let mut stacks: Stacks = Default::default();
    initial_status_rows.reverse();
    for row in initial_status_rows {
        row.into_iter()
            .enumerate()
            .filter(|(_, char)| char.is_some())
            .for_each(|(index, opt_char)| stacks[index].push(opt_char.unwrap()))
    }

    Ok((stacks, move_commands))
}

fn parse_move_command(inner_rule: Pairs<Rule>) -> Result<MoveCommand, Box<dyn Error>> {
    let mut inner_rule = inner_rule;
    let amount = inner_rule.next().unwrap().as_str().parse()?;
    let from_column = inner_rule.next().unwrap().as_str().parse()?;
    let to_column = inner_rule.next().unwrap().as_str().parse()?;

    Ok(MoveCommand::new(amount, from_column, to_column))
}

fn parse_initial_status(inner_rule: Pairs<Rule>) -> Vec<Option<char>> {
    inner_rule
        .map(|r| match r.as_rule() {
            Rule::boxLetter => Some(r.as_str().chars().next().unwrap()),
            _ => None,
        })
        .collect::<Vec<_>>()
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
    fn test_parse() {
        let result = parse_input(TEST_INPUT);
        match result {
            Err(e) => panic!("{}", e.to_string()),
            Ok(_) => {}
        }

        let (stacks, move_commands) = result.unwrap();
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(move_commands[0], MoveCommand::new(1, 2, 1))
    }
}
