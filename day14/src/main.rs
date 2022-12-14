use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn below_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn below_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

struct Cave {
    map: HashMap<Point, Content>,
}

impl Cave {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add_floor(&mut self) {
        let lowest_point = self.map.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        self.add_rock(
            Point::new(0, lowest_point + 2),
            Point::new(1001, lowest_point + 2),
        );
    }

    fn add_rock(&mut self, a: Point, b: Point) {
        if a.x == b.x {
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);
            for y in min_y..=max_y {
                self.map.insert(Point::new(a.x, y), Content::Rock);
            }
        } else {
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);

            for x in min_x..=max_x {
                self.map.insert(Point::new(x, a.y), Content::Rock);
            }
        }
    }

    fn get(&self, point: &Point) -> Option<&Content> {
        self.map.get(&point)
    }

    fn insert_sand(&mut self) -> bool {
        let mut current_position = Point::new(500, 0);
        if self.get(&current_position).is_some() {
            return false;
        }

        loop {
            if current_position.y > 1000 {
                return false;
            }
            let below = current_position.below();
            let below_left = current_position.below_left();
            let below_right = current_position.below_right();

            let positions = [below, below_left, below_right];
            let new_position = positions.into_iter().find(|p| self.get(p).is_none());

            match new_position {
                Some(position) => {
                    current_position = position;
                }
                None => break,
            }
        }

        self.map.insert(current_position, Content::Sand);
        true
    }
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut cave = Cave::new();
        input.lines().for_each(|line| {
            let parts: Vec<Point> = line
                .split(" -> ")
                .map(|p| p.split_once(",").unwrap())
                .map(|(a, b)| Point::new(a.parse().unwrap(), b.parse().unwrap()))
                .collect();
            parts.windows(2).for_each(|points| {
                cave.add_rock(points[0], points[1]);
            })
        });

        cave
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Content {
    Sand,
    Rock,
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn common_cave_part(mut cave: Cave) -> usize {
    let mut count = 0;
    while cave.insert_sand() {
        count += 1;
    }

    count
}

fn part_1(input: &str) -> usize {
    let cave = Cave::from(input);
    common_cave_part(cave)
}

fn part_2(input: &str) -> usize {
    let mut cave = Cave::from(input);
    cave.add_floor();

    common_cave_part(cave)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse_input() {
        let cave = Cave::from(TEST_INPUT);

        assert_eq!(cave.get(&Point::new(498, 4)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(498, 5)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(498, 6)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(497, 6)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(496, 6)), Some(&Content::Rock));

        assert_eq!(cave.get(&Point::new(503, 4)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(502, 4)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(502, 8)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(502, 9)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(501, 9)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(495, 9)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(494, 9)), Some(&Content::Rock));

        assert_eq!(cave.map.len(), 20)
    }

    #[test]
    fn test_insert_sand() {
        let mut cave = Cave::from(TEST_INPUT);

        assert_eq!(cave.insert_sand(), true);
    }

    #[test]
    fn test_insert_more_sand() {
        let mut cave = Cave::from(TEST_INPUT);

        for _ in 0..24 {
            assert_eq!(cave.insert_sand(), true);
        }
        assert_eq!(cave.insert_sand(), false)
    }

    #[test]
    fn test_add_floor() {
        let mut cave = Cave::from(TEST_INPUT);
        cave.add_floor();

        assert_eq!(cave.get(&Point::new(0, 11)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(500, 11)), Some(&Content::Rock));
        assert_eq!(cave.get(&Point::new(1001, 11)), Some(&Content::Rock));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 24)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 93)
    }
}
