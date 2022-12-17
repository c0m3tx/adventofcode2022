use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
const INPUT: &str = include_str!("../input.txt");

lazy_static! {
    static ref REGEX: Regex = regex::Regex::new(
        r#"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z][A-Z](?:, )?)+)"#,
    )
    .unwrap();
}

#[derive(Clone)]
struct Valve<'a> {
    name: &'a str,
    rate: usize,
    neighbors: Vec<&'a str>,
}

#[derive(Clone)]
struct CaveNetwork<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> From<&'a str> for Valve<'a> {
    fn from(input: &'a str) -> Self {
        let captures = REGEX
            .captures(input)
            .expect(format!("Invalid input: {}", input).as_str());
        let name: &'a str = captures.get(1).map(|s| s.as_str()).expect("Name not found");
        let rate = captures
            .get(2)
            .expect("Rate not found")
            .as_str()
            .parse()
            .unwrap();

        let neighbors: Vec<&'a str> = captures
            .get(3)
            .expect("Neighbors not found")
            .as_str()
            .split(", ")
            .collect();

        Valve {
            name,
            rate,
            neighbors: neighbors,
        }
    }
}

impl<'a> From<&'a str> for CaveNetwork<'a> {
    fn from(input: &'a str) -> Self {
        let mut valves = HashMap::new();
        let mut open = HashMap::new();

        for line in input.lines() {
            let valve = Valve::from(line);
            let name = valve.name;
            open.insert(name, false);
            valves.insert(name, valve);
        }

        CaveNetwork { valves }
    }
}

impl<'a> CaveNetwork<'a> {
    fn interesting_nodes(&self) -> Vec<&str> {
        self.valves
            .iter()
            .filter_map(|(name, valve)| (valve.rate > 0).then_some(*name))
            .collect()
    }

    fn shortest_path_between(&'a self, source: &'a str, destination: &'a str) -> Vec<&'a str> {
        let mut nodes_to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        let mut previous = HashMap::new();
        nodes_to_visit.push_back(source);
        visited.insert(source);

        while let Some(node) = nodes_to_visit.pop_front() {
            for neighbor in self.valves.get(node).unwrap().neighbors.iter() {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor);
                    nodes_to_visit.push_back(neighbor);
                    previous.insert(neighbor, node);
                }
            }
        }

        let mut path: Vec<&'a str> = vec![destination];
        let mut current_node = destination;
        loop {
            current_node = *previous.get(&current_node).unwrap();
            path.push(current_node);
            if current_node == source {
                break;
            }
        }
        path.reverse();

        path
    }

    fn execute_path(&self, path: &Vec<&str>) -> usize {
        let interesting_nodes = self.interesting_nodes();
        let interesting_nodes_in_path = path
            .iter()
            .filter(|name| interesting_nodes.contains(name))
            .count();
        let open_bits: usize = (1 << interesting_nodes_in_path) - 1;

        (0..=open_bits)
            .map(|open_bits| {
                let mut open_bits = open_bits.clone();
                let mut exhausted = 0;
                let mut current_time = 0;
                let mut open_valves = HashSet::new();
                for name in path.iter() {
                    if current_time > 30 {
                        break;
                    }
                    let valve = self.valves.get(name).unwrap();
                    if valve.rate > 0 && current_time <= 29 && !open_valves.contains(name) {
                        if (open_bits % 2) == 1 {
                            current_time += 1;
                            exhausted += (30 - current_time) * valve.rate;
                            open_valves.insert(*name);
                        }
                        open_bits = open_bits >> 1;
                    }
                    current_time += 1;
                }

                exhausted
            })
            .max()
            .unwrap()
    }

    fn find_best_path_to_exhaust_gas(&self) -> usize {
        let interesting_nodes: Vec<_> = self.interesting_nodes();
        let interesting_nodes_length = interesting_nodes.len();

        interesting_nodes
            .iter()
            .permutations(interesting_nodes_length)
            .map(|nodes| {
                let path_to_starting = self.shortest_path_between("AA", nodes[0]);
                let rest_of_path = nodes
                    .windows(2)
                    .flat_map(|nodes| {
                        self.shortest_path_between(nodes[0], nodes[1])
                            .into_iter()
                            .skip(1)
                    })
                    .collect();

                [path_to_starting, rest_of_path].concat()
            })
            .map(|path| self.execute_path(&path))
            .max()
            .unwrap()
    }
}

fn main() {
    println!("{}", part_1(INPUT));
}

fn part_1(input: &str) -> usize {
    let network = CaveNetwork::from(input);
    network.find_best_path_to_exhaust_gas()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_parse_valve() {
        let input = "Valve XB has flow rate=0; tunnels lead to valves YV, RP";
        let valve = Valve::from(input);

        assert_eq!(valve.name, "XB");
        assert_eq!(valve.neighbors.len(), 2);
        assert_eq!(valve.rate, 0);
    }

    #[test]
    fn test_find_path_between() {
        let network = CaveNetwork::from(TEST_INPUT);
        let path = network.shortest_path_between("AA", "CC");
        assert_eq!(path, vec!["AA", "DD", "CC"]);
    }

    #[test]
    fn test_execute_path() {
        let network = CaveNetwork::from(TEST_INPUT);
        let path = vec![
            "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
            "GG", "FF", "EE", "DD", "CC",
        ];
        assert_eq!(network.execute_path(&path), 1651);
    }

    #[test]
    fn test_path_finder() {
        let network = CaveNetwork::from(TEST_INPUT);
        assert_eq!(network.find_best_path_to_exhaust_gas(), 1651);
    }
}
