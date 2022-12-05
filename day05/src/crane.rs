use crate::types::{MoveCommand, Stacks};

pub fn v9000(stacks: Stacks, movements: Vec<MoveCommand>) -> Stacks {
    let mut stacks = stacks;

    movements.iter().for_each(|movement| {
        step_v9000(&mut stacks, movement);
    });

    stacks
}

pub fn v9001(stacks: Stacks, movements: Vec<MoveCommand>) -> Stacks {
    let mut stacks = stacks;

    movements.iter().for_each(|movement| {
        step_v9001(&mut stacks, movement);
    });

    stacks
}

fn step_v9000(stacks: &mut Stacks, movement: &MoveCommand) {
    for _ in 0..movement.amount {
        let value = stacks[movement.from_column - 1]
            .pop()
            .expect("Error while taking element");
        stacks[movement.to_column - 1].push(value);
    }
}
fn step_v9001(stacks: &mut Stacks, movement: &MoveCommand) {
    let mut temp_stack = vec![];
    for _ in 0..movement.amount {
        let value = stacks[movement.from_column - 1]
            .pop()
            .expect("Error while taking element");
        temp_stack.push(value);
    }

    for _ in 0..movement.amount {
        stacks[movement.to_column - 1].push(temp_stack.pop().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const TEST_INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    #[test]
    fn test_step_v9000() -> Result<(), Box<dyn Error>> {
        let (mut stacks, move_commands) = crate::parser::parse_input(TEST_INPUT)?;

        step_v9000(&mut stacks, move_commands.first().unwrap());

        assert_eq!(stacks[0], vec!['Z', 'N', 'D']);

        Ok(())
    }

    #[test]
    fn test_v9000() -> Result<(), Box<dyn Error>> {
        let (stacks, move_commands) = crate::parser::parse_input(TEST_INPUT)?;

        let final_stack = v9000(stacks, move_commands);

        assert_eq!(final_stack[0], vec!['Z', 'N', 'D', 'C']);
        assert_eq!(final_stack[1], vec!['M']);
        assert_eq!(final_stack[2], vec!['P']);

        Ok(())
    }

    #[test]
    fn test_step_v9001() -> Result<(), Box<dyn Error>> {
        let (mut stacks, _) = crate::parser::parse_input(TEST_INPUT)?;

        step_v9001(&mut stacks, &MoveCommand::new(2, 2, 3));

        assert_eq!(stacks[2], vec!['P', 'C', 'D']);

        Ok(())
    }

    #[test]
    fn test_v9001() -> Result<(), Box<dyn Error>> {
        let (stacks, move_commands) = crate::parser::parse_input(TEST_INPUT)?;

        let final_stack = v9001(stacks, move_commands);

        assert_eq!(final_stack[0], vec!['Z', 'N', 'D', 'M']);
        assert_eq!(final_stack[1], vec!['C']);
        assert_eq!(final_stack[2], vec!['P']);

        Ok(())
    }
}
