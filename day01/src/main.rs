use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input.lines();

    let mut elves_values = extract_elves(input)?;
    elves_values.sort();
    elves_values.reverse();
    println!("Max elf value: {}", elves_values.first().unwrap());
    println!("Sub top 3: {}", elves_values.iter().take(3).sum::<u64>());

    Ok(())
}

fn extract_elves<'a>(input: impl Iterator<Item = &'a str>) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut elves = vec![];
    let mut current = 0;
    for input_line in input {
        if input_line == "" {
            elves.push(current);
            current = 0;
        } else {
            current += input_line.parse::<u64>()?;
        }
    }

    elves.push(current);

    Ok(elves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_elf() {
        let input = "1000\n2000\n3000";
        let expected = vec![6000];

        assert_eq!(extract_elves(input.lines()).unwrap(), expected);
    }

    #[test]
    fn test_extract_multiple_elves() {
        let input = "1000\n2000\n\n3000\n4000\n5000";
        let expected = vec![3000, 12000];

        assert_eq!(extract_elves(input.lines()).unwrap(), expected);
    }
}
