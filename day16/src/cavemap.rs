use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REGEX: Regex = regex::Regex::new(
        r#"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z][A-Z](?:, )?)+)"#,
    )
    .unwrap();
}

type DistanceMatrix = Vec<Vec<usize>>;

#[derive(Clone, Debug)]
struct Valve {
    name: usize,
    rate: usize,
    neighbors: Vec<usize>,
}

#[derive(Clone)]
pub struct CaveNetwork {
    valves: Vec<Valve>,
}

impl Valve {
    fn new(name: usize, rate: usize, neighbors: Vec<usize>) -> Self {
        Valve {
            name,
            rate,
            neighbors,
        }
    }

    fn worthy(&self) -> bool {
        self.rate > 0
    }
}

impl From<&str> for CaveNetwork {
    fn from(input: &str) -> Self {
        let valve_map =
            input
                .lines()
                .enumerate()
                .fold(HashMap::new(), |mut map, (index, valve)| {
                    let name = REGEX
                        .captures(valve)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_owned();
                    map.insert(name, index);

                    map
                });

        let valves = input
            .lines()
            .map(|valve| {
                let captures = REGEX.captures(valve).unwrap();
                let name = captures[1].to_string();
                let name = valve_map.get(&name).cloned().unwrap();
                let rate = captures[2].parse().unwrap();
                let neighbors = captures[3]
                    .split(", ")
                    .map(|neigh| valve_map.get(neigh).cloned().unwrap())
                    .collect();
                Valve::new(name, rate, neighbors)
            })
            .collect();

        CaveNetwork { valves }
    }
}

impl CaveNetwork {
    fn calculate_distance_matrix(&self) -> DistanceMatrix {
        let valve_count = self.valves.len();
        let mut dist = vec![vec![usize::MAX / 2; valve_count]; valve_count];
        for valve in &self.valves {
            dist[valve.name][valve.name] = 0;
            for neighbor in &valve.neighbors {
                dist[valve.name][*neighbor] = 1;
            }
        }

        for k in 0..valve_count {
            for i in 0..valve_count {
                for j in 0..valve_count {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j]
                    }
                }
            }
        }

        dist
    }

    pub fn most_exhausted_gas(&self) -> usize {
        let distance_matrix = self.calculate_distance_matrix();

        let worthy_valves: Vec<usize> = self
            .valves
            .iter()
            // .map(|v| v.name)
            .filter_map(|v| v.worthy().then_some(v.name))
            .collect();
        self.run_sequence(&distance_matrix, worthy_valves, 0, 0, 0)
    }

    fn run_sequence(
        &self,
        distance_matrix: &DistanceMatrix,
        sequence: Vec<usize>,
        position: usize,
        exhausted: usize,
        time_passed: usize,
    ) -> usize {
        if time_passed > 30 {
            return exhausted;
        };

        let current_valve = &self.valves[position];
        if sequence.is_empty() {
            if current_valve.worthy() && time_passed < 30 {
                let exhausted_by_me = current_valve.rate * (29 - time_passed);
                return exhausted + exhausted_by_me;
            }
        }

        sequence
            .iter()
            .enumerate()
            .map(|(index, &next_neigh)| {
                let distance = distance_matrix[position][next_neigh];
                let mut new_sequence: Vec<usize> = sequence.clone();
                new_sequence.remove(index);
                if time_passed + distance > 30 {
                    if current_valve.worthy() && time_passed < 30 {
                        let exhausted_by_me = current_valve.rate * (29 - time_passed);
                        exhausted + exhausted_by_me
                    } else {
                        exhausted
                    }
                } else if current_valve.worthy() {
                    let exhausted_by_me = current_valve.rate * (29 - time_passed);
                    self.run_sequence(
                        distance_matrix,
                        new_sequence,
                        next_neigh,
                        exhausted + exhausted_by_me,
                        time_passed + distance + 1,
                    )
                } else {
                    self.run_sequence(
                        distance_matrix,
                        new_sequence,
                        next_neigh,
                        exhausted,
                        time_passed + distance,
                    )
                }
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_input_parsing() {
        let cave_network = CaveNetwork::from(TEST_INPUT);

        assert_eq!(cave_network.valves.len(), 10);
        assert_eq!(cave_network.valves[0].neighbors, vec![3, 8, 1]);
    }

    #[test]
    fn test_calculate_distance_matrix() {
        let cave_network = CaveNetwork::from(TEST_INPUT);

        let distance_matrix = cave_network.calculate_distance_matrix();
        // Distance between a valve and itself is always 0
        assert_eq!(distance_matrix[0][0], 0);
        assert_eq!(distance_matrix[0][3], 1);

        distance_matrix
            .into_iter()
            .for_each(|row| println!("{:?}", row))
    }

    #[test]
    fn test_most_exhausted_gas() {
        let cave_network = CaveNetwork::from(TEST_INPUT);
        assert_eq!(cave_network.most_exhausted_gas(), 1651)
    }

    #[test]
    fn test_run_sequence() {
        let cave_network = CaveNetwork::from(TEST_INPUT);
        let distance_matrix = cave_network.calculate_distance_matrix();

        assert_eq!(
            cave_network.run_sequence(&distance_matrix, vec![1, 2, 3, 4, 7, 9], 0, 0, 0),
            1651
        );
    }
}
