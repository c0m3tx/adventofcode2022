use std::collections::HashSet;

use crate::command::Command;
use crate::point::Point;

pub type VisitedPositions = HashSet<Point>;

pub struct SimpleRope {
    pub head: Point,
    pub tail: Point,
}

impl Default for SimpleRope {
    fn default() -> Self {
        Self {
            head: Point(0, 0),
            tail: Point(0, 0),
        }
    }
}

pub trait Rope {
    fn execute_command(&mut self, command: &Command) -> VisitedPositions;
}

impl Rope for SimpleRope {
    fn execute_command(&mut self, command: &Command) -> VisitedPositions {
        let mut visited_positions = VisitedPositions::new();

        for _ in 0..command.distance {
            self.head.move_one(&command.direction);
            if self.head.distance_from(&self.tail) >= 2.0 {
                self.tail.get_closer_to(&self.head)
            }

            visited_positions.insert(self.tail);
        }

        visited_positions
    }
}

pub struct ComplexRope {
    rope: [Point; 10],
}

impl Default for ComplexRope {
    fn default() -> Self {
        Self {
            rope: [Point::default(); 10],
        }
    }
}

impl Rope for ComplexRope {
    fn execute_command(&mut self, command: &Command) -> VisitedPositions {
        let mut visited_positions = VisitedPositions::new();

        for _ in 0..command.distance {
            self.rope[0].move_one(&command.direction);
            for i in 1..=9 {
                let reference = &self.rope[i - 1].clone();
                let current = &mut self.rope[i];
                if current.distance_from(&reference) >= 2.0 {
                    current.get_closer_to(&reference);
                }
            }
            visited_positions.insert(self.rope[9]);
        }

        visited_positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::command::Direction;

    #[test]
    fn test_execute_command_no_tail_move() {
        let mut rope = SimpleRope::default();
        rope.execute_command(&Command::new(Direction::Up, 1));

        assert_eq!(rope.head, Point(0, -1));
        assert_eq!(rope.tail, Point(0, 0));
    }

    #[test]
    fn test_execute_command_tails_follows_in_the_same_direction() {
        let mut rope = SimpleRope::default();
        rope.execute_command(&Command::new(Direction::Up, 2));
        assert_eq!(rope.head, Point(0, -2));
        assert_eq!(rope.tail, Point(0, -1));
    }
}
