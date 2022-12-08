#![allow(unused_variables)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../input.txt");

struct TreeMap {
    trees: Vec<Vec<Tree>>,
    rows: usize,
    cols: usize,
}

#[derive(Clone)]
struct Tree {
    height: isize,
    visible: bool,
    scenic_score: usize,
}

impl Tree {
    fn new(height: isize) -> Self {
        Self {
            height,
            visible: false,
            scenic_score: 0,
        }
    }
}

impl TreeMap {
    fn at(&self, row: usize, col: usize) -> &Tree {
        &self.trees[row][col]
    }

    fn mut_at(&mut self, row: usize, col: usize) -> &mut Tree {
        &mut self.trees[row][col]
    }

    fn print_visibility(&self) -> String {
        self.trees
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tree| if tree.visible { 'T' } else { 'F' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn parse(input: &str) -> TreeMap {
        let trees: Vec<Vec<Tree>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c as isize - '0' as isize)
                    .map(Tree::new)
                    .collect()
            })
            .collect();

        let rows = trees.len();
        let cols = trees[0].len();

        TreeMap { trees, rows, cols }
    }

    fn calculate_visibility(&mut self) {
        for row in 0..self.rows {
            let mut highest = -1;
            for col in 0..self.cols {
                let tree = &mut self.trees[row][col];
                if tree.height > highest {
                    tree.visible = true;
                    highest = tree.height;
                }
            }

            highest = -1;
            for col in (0..self.cols).rev() {
                let tree = &mut self.trees[row][col];
                if tree.height > highest {
                    tree.visible = true;
                    highest = tree.height;
                }
            }
        }

        for col in 0..self.cols {
            let mut highest = -1;
            for row in 0..self.rows {
                let tree = &mut self.trees[row][col];
                if tree.height > highest {
                    tree.visible = true;
                    highest = tree.height;
                }
            }

            highest = -1;
            for row in (0..self.rows).rev() {
                let tree = &mut self.trees[row][col];
                if tree.height > highest {
                    tree.visible = true;
                    highest = tree.height;
                }
            }
        }
    }

    fn calculate_scenic_scores(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let score = self.calculate_scenic_score_for_tree(row, col);
                println!("For row {}, column {}, score is {}", row, col, score);
                self.mut_at(row, col).scenic_score = score;
            }
        }
    }

    fn scenic_score_for_tree_line<'a>(
        this_height: usize,
        mut tree_iter: impl Iterator<Item = &'a Tree>,
    ) -> usize {
        let mut count = 0;
        while let Some(tree) = tree_iter.next() {
            count += 1;

            if tree.height >= this_height as isize {
                return count;
            }
        }

        count
    }

    fn calculate_scenic_score_for_tree(&mut self, row: usize, col: usize) -> usize {
        let this_height = self.at(row, col).height as usize;

        let going_up = (0..row).rev().map(|row| self.at(row, col));
        let going_up = Self::scenic_score_for_tree_line(this_height, going_up);

        let going_down = ((row + 1)..self.rows).map(|row| self.at(row, col));
        let going_down = Self::scenic_score_for_tree_line(this_height, going_down);

        let going_left = (0..col).rev().map(|c| self.at(row, c));
        let going_left = Self::scenic_score_for_tree_line(this_height, going_left);

        let going_right = ((col + 1)..self.cols).map(|c| self.at(row, c));
        let going_right = Self::scenic_score_for_tree_line(this_height, going_right);

        going_up * going_down * going_left * going_right
    }

    fn find_highest_scenic_score(&self) -> usize {
        (0..(self.rows))
            .flat_map(|row| self.trees[row].iter().map(|t| t.scenic_score))
            .max()
            .unwrap()
    }

    fn count_visibles(&self) -> usize {
        (0..(self.rows))
            .flat_map(|row| self.trees[row].iter().filter(|t| t.visible))
            .count()
    }
}

fn part_1(treemap: &mut TreeMap) -> usize {
    treemap.calculate_visibility();
    treemap.count_visibles()
}

fn part_2(treemap: &mut TreeMap) -> usize {
    treemap.calculate_scenic_scores();
    treemap.find_highest_scenic_score()
}

fn main() {
    let mut treemap = TreeMap::parse(INPUT);
    println!("Part 1: {}", part_1(&mut treemap));
    println!("Part 2: {}", part_2(&mut treemap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_calculate_visibility() {
        let mut map = TreeMap::parse(TEST_INPUT);
        map.calculate_visibility();

        println!("{}", map.print_visibility());

        assert_eq!(map.trees[0][0].visible, true);
        assert_eq!(map.trees[1][1].visible, true);
        assert_eq!(map.trees[1][3].visible, false);
    }

    #[test]
    fn test_part_1() {
        let mut treemap = TreeMap::parse(TEST_INPUT);
        let visibles = part_1(&mut treemap);
        assert_eq!(visibles, 21);
    }

    #[test]
    fn test_calculate_scenic_score() {
        let mut map = TreeMap::parse(TEST_INPUT);
        map.calculate_visibility();

        println!("{}", map.print_visibility());

        let scenic_score = map.calculate_scenic_score_for_tree(3, 2);
        assert_eq!(scenic_score, 8)
    }

    #[test]
    fn test_part_2() {
        let mut map = TreeMap::parse(TEST_INPUT);
        map.calculate_visibility();
        let max_scenic_score = part_2(&mut map);
        assert_eq!(max_scenic_score, 8);
    }
}
