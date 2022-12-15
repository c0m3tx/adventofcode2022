use std::{collections::HashSet, ops::RangeInclusive};

type Point = common::Point<isize>;

const INPUT: &str = include_str!("../input.txt");

struct Sensor {
    center: Point,
    beacon: Point,
    radius: usize,
}

impl Sensor {
    fn just_outside_of_border(&self) -> Vec<Point> {
        let mut output = vec![];
        let north = Point::new(self.center.x, self.center.y - self.radius as isize - 1);
        let east = Point::new(self.center.x + self.radius as isize + 1, self.center.y);
        let west = Point::new(self.center.x - self.radius as isize - 1, self.center.y);
        let south = Point::new(self.center.x, self.center.y + self.radius as isize + 1);

        let mut current_point = north.clone();
        while current_point != east {
            current_point = current_point.below_right();
            output.push(current_point.clone());
        }
        while current_point != south {
            current_point = current_point.below_left();
            output.push(current_point.clone());
        }
        while current_point != west {
            current_point = current_point.above_left();
            output.push(current_point.clone());
        }
        while current_point != north {
            current_point = current_point.above_right();
            output.push(current_point.clone());
        }

        output
    }

    fn can_see(&self, point: &Point) -> bool {
        self.center.cab_distance(&point) <= self.radius
    }
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Self {
        regex::Regex::new(
            "Sensor at x=([^,]+), y=([^:]+): closest beacon is at x=([^,]+), y=([^ ]+)",
        )
        .unwrap()
        .captures(input)
        .map(|captures| {
            let center: Point =
                Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());
            let beacon = Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap());
            let radius = center.cab_distance(&beacon);
            Self {
                center,
                radius,
                beacon,
            }
        })
        .unwrap_or_else(|| panic!("Invalid input: {}", input))
    }
}

fn part_1(input: &str, row: isize) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|line| line.into()).collect();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    sensors.iter().for_each(|s| {
        min_x = min_x.min(s.center.x - s.radius as isize);
        max_x = max_x.max(s.center.x + s.radius as isize);
    });

    let beacons_on_line = sensors
        .iter()
        .filter_map(|s| (s.beacon.y == row).then_some(s.beacon))
        .collect::<HashSet<_>>()
        .into_iter()
        .count();

    ((min_x)..=(max_x))
        .filter_map(|x| {
            let point = Point::new(x, row);
            sensors
                .iter()
                .any(|s| s.center.cab_distance(&point) <= s.radius)
                .then_some(point)
        })
        .count()
        - beacons_on_line
}

fn part_2(input: &str, range: RangeInclusive<isize>) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|line| line.into()).collect();

    sensors
        .iter()
        .flat_map(|s| s.just_outside_of_border())
        .filter(|p| range.contains(&p.x) && range.contains(&p.y))
        .find_map(|point| {
            sensors
                .iter()
                .all(|s| !s.can_see(&point))
                .then_some(point.x as usize * 4000000 + point.y as usize)
        })
        .unwrap_or(0)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT, 2000000));
    println!("Part 2: {}", part_2(INPUT, RangeInclusive::new(0, 4000000)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_parse_sensor() {
        let sensor: Sensor = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".into();
        assert_eq!(sensor.center, Point::new(2, 18));
        assert_eq!(sensor.radius, 7);
    }

    #[test]
    fn test_just_outside_of_border() {
        let sensor: Sensor = Sensor {
            center: Point::new(0, 0),
            beacon: Point::new(1, 3),
            radius: 2,
        };
        assert_eq!(sensor.just_outside_of_border().len(), 12);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT, 0..=20), 56000011);
    }
}
