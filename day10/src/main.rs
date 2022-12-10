const INPUT: &str = include_str!("../input.txt");

struct Cpu {
    x: isize,
    cycles_history: Vec<isize>,
}

impl Cpu {
    fn execute(&mut self, instructions: &str) {
        instructions.lines().for_each(|f| {
            if f == "noop" {
                self.noop();
            } else {
                let (_, amount) = f.split_once(" ").unwrap();
                let amount: isize = amount.parse().unwrap();
                self.add_x(amount);
            }
        });
        self.noop();
    }

    fn new() -> Self {
        Self {
            x: 1,
            cycles_history: Vec::new(),
        }
    }

    fn noop(&mut self) {
        self.cycles_history.push(self.x);
    }

    fn add_x(&mut self, x: isize) {
        self.noop();
        self.noop();
        self.x += x;
    }

    fn crt(&self) -> String {
        let mut crt = [['.'; 40]; 6];
        for row in 0..6 {
            for clock_cycle in 0..40 {
                let value = self
                    .cycles_history
                    .get(row * 40 + clock_cycle)
                    .expect("Too few cycles");
                if (value - 1..=value + 1).contains(&(clock_cycle as isize)) {
                    crt[row][clock_cycle] = '#'
                }
            }
        }

        crt.iter()
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn part_1(input: &str) -> isize {
    let mut cpu = Cpu::new();
    cpu.execute(input);

    let mut total = 0;

    for n in [20, 60, 100, 140, 180, 220] {
        let nth_value = cpu.cycles_history.iter().nth(n - 1).unwrap();
        total += *nth_value * (n as isize);
    }

    total
}

fn part_2(input: &str) -> String {
    let mut cpu = Cpu::new();
    cpu.execute(input);
    cpu.crt()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: \n{}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_TEST_INPUT: &str = "noop\naddx 3\naddx -5";
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_cpu_noop() {
        let mut cpu = Cpu::new();
        cpu.noop();
        assert_eq!(cpu.x, 1);
        assert_eq!(cpu.cycles_history.len(), 1)
    }

    #[test]
    fn test_cpu_add_x() {
        let mut cpu = Cpu::new();
        cpu.add_x(3);
        assert_eq!(cpu.x, 4);
        assert_eq!(cpu.cycles_history, vec![1, 1]);
    }

    #[test]
    fn test_minimal_input() {
        let mut cpu = Cpu::new();
        cpu.execute(MINIMAL_TEST_INPUT);
        assert_eq!(cpu.cycles_history, vec![1, 1, 1, 4, 4, -1])
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 13140)
    }

    const EXPECTED_PART_2: &str = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), EXPECTED_PART_2)
    }
}
