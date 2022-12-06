use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!(
        "Part 1: {}",
        find_marker(INPUT, 4).expect("Marker not found")
    );

    println!(
        "Part 2: {}",
        find_marker(INPUT, 14).expect("Marker not found")
    )
}

fn find_marker(input: &str, length: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find_map(|(i, window)| all_different(window).then_some(i + length))
}

fn all_different(chars: &[u8]) -> bool {
    chars.iter().cloned().collect::<HashSet<u8>>().len() == chars.len()
}

// so... this is just for fun, I'm not even sure it's actually more efficient
fn unsafe_find_marker(input: &str, length: usize) -> Option<usize> {
    let mut charmap: HashMap<char, u64> = HashMap::new();
    let mut head: *const u8 = input.as_ptr();
    let mut tail: *const u8 = input.as_ptr();
    for _ in 0..length {
        let current_char = unsafe { *tail };
        charmap
            .entry(current_char as char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        tail = unsafe { tail.add(1) };
    }

    tail = unsafe { tail.sub(1) };

    for i in 0..(input.len() - length) {
        if charmap.keys().len() == length {
            return Some(i + length);
        }
        let (head_char, tail_char) = unsafe {
            let head_char = *head as char;
            head = head.add(1);
            tail = tail.add(1);
            let tail_char = *tail as char;
            (head_char, tail_char)
        };

        let count = charmap.get(&head_char).unwrap();
        if *count == 1 {
            charmap.remove(&head_char);
        } else {
            charmap.insert(head_char, count - 1);
        }

        charmap
            .entry(tail_char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_1() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    }

    #[test]
    fn test_marker_2() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    }

    #[test]
    fn test_marker_3() {
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }

    #[test]
    fn test_marker_4() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
    }

    #[test]
    fn test_message_1() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
    }

    #[test]
    fn test_message_2() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
    }

    #[test]
    fn test_message_3() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
    }

    #[test]
    fn test_message_4() {
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
    }

    #[test]
    fn test_message_5() {
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }

    #[test]
    fn test_unsafe_marker_1() {
        assert_eq!(
            unsafe_find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
    }

    #[test]
    fn test_unsafe_marker_2() {
        assert_eq!(
            unsafe_find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
    }

    #[test]
    fn test_unsafe_marker_3() {
        assert_eq!(
            unsafe_find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }

    #[test]
    fn test_unsafe_marker_4() {
        assert_eq!(
            unsafe_find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }
}
