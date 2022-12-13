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

fn visit_neighbor<F>(
    input: &mut Matrix2D<Location>,
    (row, col): (isize, isize),
    my_distance: usize,
    my_height: usize,
    condition: F,
) -> Option<(isize, isize)>
where
    F: Fn(usize, usize) -> bool,
{
    input.get_mut(row, col).and_then(|mut neigh| {
        if neigh.min_distance > my_distance + 1 && condition(neigh.height, my_height) {
            neigh.min_distance = my_distance + 1;
            Some((row, col))
        } else {
            None
        }
    })
}

fn bfs<F>(input: &mut Matrix2D<Location>, (start_row, start_col): (usize, usize), condition: F)
where
    F: Fn(usize, usize) -> bool,
{
    let mut input = input;
    let mut coords_to_visit: VecDeque<(isize, isize)> = VecDeque::new();

    input
        .get_mut(start_row as isize, start_col as isize)
        .unwrap()
        .min_distance = 0;
    coords_to_visit.push_back((start_row as isize, start_col as isize));

    while let Some((row, col)) = coords_to_visit.pop_front() {
        let me = input.get(row, col).unwrap();
        let my_distance = me.min_distance;
        let my_height = me.height;

        let neighbors = [
            (row, col - 1),
            (row, col + 1),
            (row - 1, col),
            (row + 1, col),
        ];
        neighbors.into_iter().for_each(|neigh| {
            visit_neighbor(&mut input, neigh, my_distance, my_height, &condition)
                .map(|c| coords_to_visit.push_back(c));
        });
    }
}

fn part_1(input: &str) -> usize {
    let mut input = parse_input(input);
    let (start_row, start_col) = input.find(|l| l.kind == LocationKind::Start).unwrap();
    let (end_row, end_col) = input.find(|l| l.kind == LocationKind::End).unwrap();

    bfs(
        &mut input,
        (start_row, start_col),
        |neigh_height, my_height| (neigh_height as isize - my_height as isize) <= 1,
    );

    input.get(end_row, end_col).unwrap().min_distance
}

fn part_2(input: &str) -> usize {
    let mut input = parse_input(input);
    let (start_row, start_col) = input.find(|l| l.kind == LocationKind::End).unwrap();

    bfs(
        &mut input,
        (start_row, start_col),
        |neigh_height, my_height| (my_height as isize - neigh_height as isize) <= 1,
    );

    input
        .iter()
        .filter(|l| l.height == 1)
        .map(|l| l.min_distance)
        .min()
        .unwrap()
}

// this is just for fun
fn part_2_parallel(input: &str) -> usize {
    let input = parse_input(input);
    let starting_positions = input.find_all(|l| l.height == 1);
    let mut threads = vec![];

    for (row, col) in starting_positions {
        let mut input = input.clone();
        threads.push(thread::spawn(move || {
            bfs(&mut input, (row, col), |neigh_height, my_height| {
                (neigh_height as isize - my_height as isize) <= 1
            });
            input
                .iter()
                .filter(|l| l.kind == LocationKind::End)
                .map(|l| l.min_distance)
                .min()
                .unwrap()
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
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 29)
    }

    #[test]
    fn test_part_2_parallel() {
        assert_eq!(part_2_parallel(TEST_INPUT), 29)
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(input.cols(), 8);
        assert_eq!(input.rows(), 5);
    }
}
