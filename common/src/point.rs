#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, x: isize, y: isize) -> Self {
        Self::new(self.x + x, self.y + y)
    }

    pub fn below(&self) -> Self {
        self.shift(0, 1)
    }

    pub fn below_left(&self) -> Self {
        self.shift(-1, 1)
    }

    pub fn below_right(&self) -> Self {
        self.shift(1, 1)
    }

    pub fn above(&self) -> Self {
        self.shift(0, -1)
    }

    pub fn above_left(&self) -> Self {
        self.shift(-1, -1)
    }

    pub fn above_right(&self) -> Self {
        self.shift(1, -1)
    }

    pub fn left(&self) -> Self {
        self.shift(-1, 0)
    }

    pub fn right(&self) -> Self {
        self.shift(1, 0)
    }
}
