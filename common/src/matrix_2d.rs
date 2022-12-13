use std::fmt::Debug;
use std::fmt::Display;

pub struct Matrix2D<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix2D<T> {
    #[inline]
    fn index_for(&self, row: usize, col: usize) -> Option<usize> {
        (row < self.rows && col < self.cols).then_some(row * self.cols + col)
    }

    #[inline]
    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    pub fn get<I>(&self, row: I, col: I) -> Option<&T>
    where
        I: TryInto<usize>,
    {
        let row = row.try_into().ok()?;
        let col = col.try_into().ok()?;
        self.index_for(row, col).map(|index| &self.data[index])
    }

    pub fn get_mut<I>(&mut self, row: I, col: I) -> Option<&mut T>
    where
        I: TryInto<usize>,
    {
        let row = row.try_into().ok()?;
        let col = col.try_into().ok()?;
        self.index_for(row, col).map(|index| &mut self.data[index])
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn map<F, U>(&self, f: F) -> Matrix2D<U>
    where
        F: FnMut(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();
        Matrix2D {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    pub fn transpose(&self) -> Matrix2D<&T> {
        let mut data = Vec::with_capacity(self.data.len());

        for row in 0..self.rows {
            for col in 0..self.cols {
                data[self.index_for(row, col).unwrap()] = self.get(col, row).unwrap();
            }
        }

        Matrix2D {
            rows: self.cols,
            cols: self.rows,
            data: data,
        }
    }

    pub fn find<F>(&self, f: F) -> Option<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter().enumerate().find_map(|(index, e)| {
            if f(e) {
                Some(self.index_to_coords(index))
            } else {
                None
            }
        })
    }

    pub fn find_all<F>(&self, f: F) -> Vec<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, e)| {
                if f(e) {
                    Some(self.index_to_coords(index))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
}

impl<T: Clone> Clone for Matrix2D<T> {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix2D<T> {
    fn from(input: Vec<Vec<T>>) -> Matrix2D<T> {
        let rows = input.len();
        let cols = input[0].len();

        let data = input
            .into_iter()
            .flat_map(|row| row.into_iter().map(|elem| elem).collect::<Vec<_>>())
            .collect();

        Matrix2D { rows, cols, data }
    }
}

impl From<&str> for Matrix2D<char> {
    fn from(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = data.len();
        let cols = data[0].len();

        Matrix2D {
            rows,
            cols,
            data: data.into_iter().flat_map(|row| row.into_iter()).collect(),
        }
    }
}

impl<T: Display> Display for Matrix2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.get(row, col).unwrap())?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl<T: Debug> Debug for Matrix2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:?} ", self.get(row, col).unwrap())?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
