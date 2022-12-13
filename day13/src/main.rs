use packet::{Packet, ParseError};

mod packet;

const INPUT: &str = include_str!("../input.txt");

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, packet_pair)| {
            let (left, right) = packet_pair.split_once("\n").unwrap();
            let left: Packet = left.try_into().expect("Unable to parse packet");
            let right: Packet = right.try_into().expect("Unable to parse packet");

            if left < right {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> Result<usize, ParseError> {
    let mut packets: Vec<Packet> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    packets.push(Packet::List(vec![Packet::Value(2)]));
    packets.push(Packet::List(vec![Packet::Value(6)]));

    packets.sort();

    let value = packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if packet == &Packet::List(vec![Packet::Value(2)])
                || packet == &Packet::List(vec![Packet::Value(6)])
            {
                Some(index + 1)
            } else {
                None
            }
        })
        .product::<usize>();

    Ok(value)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT).unwrap(), 140);
    }
}
