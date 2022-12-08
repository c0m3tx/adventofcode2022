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

    fn calculate_tree_visible(&mut self, row: usize, col: usize) {
        let this_tree = self.at(row, col);
        let row_mapper = |row: usize| self.at(row, col);
        let col_mapper = |col: usize| self.at(row, col);
        let all_trees_are_lower = |t: &Tree| t.height < this_tree.height;

        let visible = (0..row).map(row_mapper).all(all_trees_are_lower)
            || (row + 1..self.rows)
                .map(row_mapper)
                .all(all_trees_are_lower)
            || (0..col).map(col_mapper).all(all_trees_are_lower)
            || (col + 1..self.cols)
                .map(col_mapper)
                .all(all_trees_are_lower);

        if visible {
            self.mut_at(row, col).visible = true;
        }
    }

    fn calculate_visibility(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.calculate_tree_visible(row, col)
            }
        }
    }

    fn calculate_scenic_scores(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let score = self.calculate_scenic_score_for_tree(row, col);
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

        assert_eq!(map.at(0, 0).visible, true);
        assert_eq!(map.at(1, 1).visible, true);
        assert_eq!(map.at(1, 3).visible, false);
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
