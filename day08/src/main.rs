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

impl From<&str> for TreeMap {
    fn from(input: &str) -> TreeMap {
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
}

impl TreeMap {
    fn print_visibility(&self) {
        let output = self
            .trees
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tree| if tree.visible { 'T' } else { 'F' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", output);
    }

    fn at(&self, row: usize, col: usize) -> &Tree {
        &self.trees[row][col]
    }

    fn mut_at(&mut self, row: usize, col: usize) -> &mut Tree {
        &mut self.trees[row][col]
    }

    fn all_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows.clone();
        let cols = self.cols.clone();
        (0..rows).flat_map(move |row| (0..cols).map(move |col| (row, col)))
    }

    fn right_from(&self, row: usize, col: usize) -> impl Iterator<Item = &Tree> {
        ((col + 1)..self.cols).map(move |col| self.at(row, col))
    }

    fn left_from(&self, row: usize, col: usize) -> impl Iterator<Item = &Tree> {
        (0..col).rev().map(move |col| self.at(row, col))
    }

    fn up_from(&self, row: usize, col: usize) -> impl Iterator<Item = &Tree> {
        (0..row).rev().map(move |row| self.at(row, col))
    }

    fn down_from(&self, row: usize, col: usize) -> impl Iterator<Item = &Tree> {
        ((row + 1)..self.rows).map(move |row| self.at(row, col))
    }

    fn is_tree_visible(&mut self, row: usize, col: usize) -> bool {
        let this_tree = self.at(row, col);
        let lower = |t: &Tree| t.height < this_tree.height;

        self.up_from(row, col).all(lower)
            || self.down_from(row, col).all(lower)
            || self.right_from(row, col).all(lower)
            || self.left_from(row, col).all(lower)
    }

    fn calculate_visibility(&mut self) {
        for (row, col) in self.all_coords() {
            self.mut_at(row, col).visible = self.is_tree_visible(row, col)
        }
    }

    fn calculate_scenic_scores(&mut self) {
        for (row, col) in self.all_coords() {
            self.mut_at(row, col).scenic_score = self.calculate_scenic_score_for_tree(row, col);
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

        let going_up = Self::scenic_score_for_tree_line(this_height, self.up_from(row, col));
        let going_down = Self::scenic_score_for_tree_line(this_height, self.down_from(row, col));
        let going_left = Self::scenic_score_for_tree_line(this_height, self.left_from(row, col));
        let going_right = Self::scenic_score_for_tree_line(this_height, self.right_from(row, col));

        going_up * going_down * going_left * going_right
    }

    fn find_highest_scenic_score(&self) -> usize {
        self.trees
            .iter()
            .flat_map(|row| row.iter().map(|t| t.scenic_score))
            .max()
            .unwrap()
    }

    fn count_visibles(&self) -> usize {
        self.trees
            .iter()
            .flat_map(|row| row.iter().filter(|t| t.visible))
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
    let mut treemap = INPUT.into();
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
    fn test_visible_tree() {
        let mut treemap: TreeMap = TEST_INPUT.into();
        assert_eq!(treemap.is_tree_visible(2, 1), true)
    }

    #[test]
    fn test_calculate_visibility() {
        let mut treemap: TreeMap = TEST_INPUT.into();
        treemap.calculate_visibility();

        assert_eq!(treemap.at(0, 0).visible, true);
        assert_eq!(treemap.at(1, 1).visible, true);
        assert_eq!(treemap.at(1, 3).visible, false);
    }

    #[test]
    fn test_part_1() {
        let mut treemap = TEST_INPUT.into();
        let visibles = part_1(&mut treemap);
        assert_eq!(visibles, 21);
    }

    #[test]
    fn test_calculate_scenic_score() {
        let mut treemap: TreeMap = TEST_INPUT.into();
        treemap.calculate_visibility();

        let scenic_score = treemap.calculate_scenic_score_for_tree(3, 2);
        assert_eq!(scenic_score, 8)
    }

    #[test]
    fn test_part_2() {
        let mut treemap: TreeMap = TEST_INPUT.into();
        treemap.calculate_visibility();
        let max_scenic_score = part_2(&mut treemap);
        assert_eq!(max_scenic_score, 8);
    }

    #[test]
    fn test_full_part_1() {
        let mut treemap = INPUT.into();
        let visibles = part_1(&mut treemap);
        assert_eq!(visibles, 1787);
    }

    #[test]
    fn test_full_part_2() {
        let mut treemap: TreeMap = INPUT.into();
        treemap.calculate_visibility();
        let max_scenic_score = part_2(&mut treemap);
        assert_eq!(max_scenic_score, 440640);
    }
}
