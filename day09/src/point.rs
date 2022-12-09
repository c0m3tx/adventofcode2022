use crate::command::Direction;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub isize, pub isize);

impl Default for Point {
    fn default() -> Self {
        Point(0, 0)
    }
}

impl Point {
    pub fn move_one(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.1 -= 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }

    pub fn distance_from(&self, other: &Point) -> f64 {
        let dx = (other.0 - self.0) as f64;
        let dy = (other.1 - self.1) as f64;

        (dx.powf(2.0) + dy.powf(2.0)).sqrt()
    }

    pub fn get_closer_to(&mut self, other: &Point) {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;

        let dx = dx.checked_div(dx.abs()).unwrap_or(0);
        let dy = dy.checked_div(dy.abs()).unwrap_or(0);

        self.0 += dx;
        self.1 += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        assert!(Point(0, 0).distance_from(&Point(1, 0)) == 1.0);
        assert!(Point(0, 0).distance_from(&Point(1, 1)) > 1.0);
        assert!(Point(0, 0).distance_from(&Point(1, 1)) < 2.0);
        assert!(Point(0, 0).distance_from(&Point(1, 2)) > 2.0);
    }

    macro_rules! test_go_closer {
        (($x1:literal, $y1:literal) -> ($x2:literal, $y2:literal) goes to ($r1:literal, $r2:literal)) => {
            let target = Point($x2, $y2);
            let mut point = Point($x1, $y1);
            point.get_closer_to(&target);
            assert_eq!(point, Point($r1, $r2))
        };
    }

    #[test]
    fn test_go_closer() {
        test_go_closer!((0, 0) -> (1, 0) goes to (1, 0));
        test_go_closer!((0, 0) -> (2, 1) goes to (1, 1));
        test_go_closer!((0, 0) -> (1, 2) goes to (1, 1));
        test_go_closer!((0, 0) -> (-1, 2) goes to (-1, 1));
        test_go_closer!((0, 0) -> (-2, 1) goes to (-1, 1));
        test_go_closer!((0, 0) -> (-2, 0) goes to (-1, 0));
    }
}
