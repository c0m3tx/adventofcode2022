use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug)]
enum ParseError {
    InvalidDigit(String),
    NoPacketFound,
    InvalidPacket,
}

#[derive(Parser)]
#[grammar = "parser.pest"]
struct PacketParser;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(usize),
}

impl TryFrom<&str> for Packet {
    type Error = ParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        PacketParser::parse(Rule::list, line)
            .map_err(|_| ParseError::InvalidPacket)?
            .next()
            .ok_or_else(|| ParseError::NoPacketFound)
            .and_then(|r| r.try_into())
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for Packet {
    type Error = ParseError;
    fn try_from(rule: Pair<Rule>) -> Result<Self, Self::Error> {
        match rule.as_rule() {
            Rule::number => {
                let value = rule
                    .as_str()
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidDigit(rule.as_str().to_owned()))?;
                Ok(Packet::Value(value))
            }
            Rule::list => {
                let mut packets = vec![];
                for packet in rule.into_inner() {
                    let inner: Packet = packet.try_into()?;
                    packets.push(inner);
                }
                Ok(Packet::List(packets))
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Packet::*;
    use super::*;

    #[test]
    fn parse_simple_list() -> Result<(), ParseError> {
        let parsed_packet: Packet = "[1,1,3,1,1]".try_into()?;
        assert_eq!(
            parsed_packet,
            List(vec![Value(1), Value(1), Value(3), Value(1), Value(1)])
        );

        Ok(())
    }

    #[test]
    fn parse_nested_lists() -> Result<(), ParseError> {
        let parsed_packet: Packet = "[[1],[2,3,4]]".try_into()?;
        assert_eq!(
            parsed_packet,
            List(vec![
                List(vec![Value(1)]),
                List(vec![Value(2), Value(3), Value(4)])
            ])
        );

        Ok(())
    }

    #[test]
    fn parse_mixed_list() -> Result<(), ParseError> {
        let parsed_packet: Packet = "[[4,4],4,4]".try_into()?;
        assert_eq!(
            parsed_packet,
            List(vec![List(vec![Value(4), Value(4)]), Value(4), Value(4),])
        );

        Ok(())
    }

    #[test]
    fn parse_empty_list() -> Result<(), ParseError> {
        let parsed_packet: Packet = "[]".try_into()?;
        assert_eq!(parsed_packet, List(vec![]));

        Ok(())
    }

    #[test]
    fn parse_nested_empty_list() -> Result<(), ParseError> {
        let parsed_packet: Packet = "[[[]]]".try_into()?;
        assert_eq!(parsed_packet, List(vec![List(vec![List(vec![])])]));

        Ok(())
    }
}
