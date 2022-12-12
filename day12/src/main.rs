use std::collections::VecDeque;
use std::thread;

use common::Matrix2D;

#[derive(PartialEq, Clone)]
enum LocationKind {
    Start,
    End,
    Normal,
}

#[derive(Clone)]
struct Location {
    height: usize,
    kind: LocationKind,
    min_distance: usize,
}

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Matrix2D<Location> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| {
                    let (height, kind) = match c {
                        'S' => (1, LocationKind::Start),
                        'E' => (26, LocationKind::End),
                        _ => (c as usize - 'a' as usize + 1, LocationKind::Normal),
                    };
                    Location {
                        height,
                        kind,
                        min_distance: usize::MAX,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into()
}

fn visit_neighbor(
    input: &mut Matrix2D<Location>,
    (row, col): (isize, isize),
    my_distance: usize,
    my_height: usize,
) -> Option<(isize, isize)> {
    input.get_mut(row, col).and_then(|mut neigh| {
        if neigh.min_distance > my_distance + 1 && (neigh.height as isize - my_height as isize) <= 1
        {
            neigh.min_distance = my_distance + 1;
            Some((row, col))
        } else {
            None
        }
    })
}

fn find_shortest(input: Matrix2D<Location>, (start_row, start_col): (isize, isize)) -> usize {
    let mut input = input;
    let (end_row, end_col) = input.find(|l| l.kind == LocationKind::End).unwrap();
    let mut coords_to_visit: VecDeque<(isize, isize)> = VecDeque::new();

    input.get_mut(start_row, start_col).unwrap().min_distance = 0;
    coords_to_visit.push_back((start_row, start_col));

    while let Some((row, col)) = coords_to_visit.pop_front() {
        let me = input.get(row, col).unwrap();
        let my_distance = me.min_distance;
        let my_height = me.height;

        visit_neighbor(&mut input, (row, col - 1), my_distance, my_height)
            .map(|c| coords_to_visit.push_back(c));
        visit_neighbor(&mut input, (row, col + 1), my_distance, my_height)
            .map(|c| coords_to_visit.push_back(c));
        visit_neighbor(&mut input, (row - 1, col), my_distance, my_height)
            .map(|c| coords_to_visit.push_back(c));
        visit_neighbor(&mut input, (row + 1, col), my_distance, my_height)
            .map(|c| coords_to_visit.push_back(c));
    }

    input.get(end_row, end_col).unwrap().min_distance
}

fn part_1(input: &str) -> usize {
    let input = parse_input(input);
    let (start_row, start_col) = input.find(|l| l.kind == LocationKind::Start).unwrap();

    find_shortest(input, (start_row as isize, start_col as isize))
}

fn part_2(input: &str) -> usize {
    let input = parse_input(input);
    let starting_positions = input.find_all(|l| l.height == 1);
    let mut threads = vec![];

    for (row, col) in starting_positions {
        let input = input.clone();
        threads.push(thread::spawn(move || {
            find_shortest(input, (row as isize, col as isize))
        }));
    }

    threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 31)
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT);
        println!("{}", input.map(|l| format!("{:4}", l.height)));
    }
}
