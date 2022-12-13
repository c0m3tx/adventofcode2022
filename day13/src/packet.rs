use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug)]
pub enum ParseError {
    InvalidDigit(String),
    NoPacketFound,
    InvalidPacket,
}

#[derive(Parser)]
#[grammar = "parser.pest"]
struct PacketParser;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub enum Packet {
    List(Vec<Packet>),
    Value(usize),
}

impl Packet {
    fn to_list(&self) -> Packet {
        match self {
            Packet::List(_) => self.clone(),
            Packet::Value(_) => Packet::List(vec![self.clone()]),
        }
    }
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

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a_iter = a.iter();
                let mut b_iter = b.iter();

                loop {
                    let a_value = a_iter.next();
                    let b_value = b_iter.next();
                    match (a_value, b_value) {
                        (Some(a), Some(b)) => {
                            let cmp = a.partial_cmp(b);
                            if cmp != Some(std::cmp::Ordering::Equal) {
                                return cmp;
                            }
                        }
                        (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                        (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                        (None, None) => return Some(std::cmp::Ordering::Equal),
                    }
                }
            }
            (a, b) => a.to_list().partial_cmp(&b.to_list()),
        }
    }
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

    macro_rules! assert_order {
        ($a:literal $op:tt $b:literal) => {{
            let packet_1: Packet = $a.try_into()?;
            let packet_2: Packet = $b.try_into()?;

            assert!(packet_1 $op packet_2);

            Ok(())
        }};
    }

    #[test]
    fn test_ordering_simple_arrays() -> Result<(), ParseError> {
        assert_order!("[1,1,3,1,1]" < "[1,1,5,1,1]")
    }

    #[test]
    fn test_ordering_complex_strucutres() -> Result<(), ParseError> {
        assert_order!("[[1],[2,3,4]]" < "[[1],4]")
    }

    #[test]
    fn test_ordering_different_length() -> Result<(), ParseError> {
        assert_order!("[9]" > "[[8,7,6]]")
    }

    #[test]
    fn test_ordering_lists_with_lists() -> Result<(), ParseError> {
        assert_order!("[[4,4],4,4]" < "[[4,4],4,4,4]")
    }

    #[test]
    fn test_ordering_different_length_arrays() -> Result<(), ParseError> {
        assert_order!("[7,7,7,7]" > "[7,7,7]")
    }

    #[test]
    fn test_empty_vs_something() -> Result<(), ParseError> {
        assert_order!("[]" < "[3]")
    }

    #[test]
    fn test_ordering_differently_nested_lists() -> Result<(), ParseError> {
        assert_order!("[[[]]]" > "[[]]")
    }

    #[test]
    fn test_really_complex_structure() -> Result<(), ParseError> {
        assert_order!("[1,[2,[3,[4,[5,6,7]]]],8,9]" > "[1,[2,[3,[4,[5,6,0]]]],8,9]")
    }
}
