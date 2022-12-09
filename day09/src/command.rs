#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl From<&str> for Direction {
    fn from(dir: &str) -> Self {
        match dir {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub direction: Direction,
    pub distance: usize,
}

impl Command {
    pub fn new(direction: Direction, distance: usize) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl From<&str> for Command {
    fn from(str: &str) -> Self {
        let (dir, distance) = str.split_once(" ").unwrap();
        Command::new(dir.into(), distance.parse().unwrap())
    }
}

pub fn parse_commands(input: &str) -> Vec<Command> {
    input.lines().map(Into::into).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    #[test]
    fn test_parse_commands() {
        let commands = parse_commands(TEST_INPUT);
        assert_eq!(commands.len(), 8);
        assert_eq!(
            commands[0],
            Command {
                direction: Direction::Right,
                distance: 4
            }
        )
    }
}
