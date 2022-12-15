#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

macro_rules! impl_ints_methods {
    ($($t:ty),*) => {
        $(
        impl Point<$t> {
            pub fn shift(&self, x: $t, y: $t) -> Self {
                Self::new(self.x + x, self.y + y)
            }

            pub fn above_left(&self) -> Self {
                Self::new(self.x - 1, self.y - 1)
            }

            pub fn above(&self) -> Self {
                Self::new(self.x, self.y - 1)
            }

            pub fn above_right(&self) -> Self {
                Self::new(self.x + 1, self.y - 1)
            }

            pub fn left(&self) -> Self {
                Self::new(self.x - 1, self.y)
            }

            pub fn right(&self) -> Self {
                Self::new(self.x + 1, self.y)
            }

            pub fn below_left(&self) -> Self {
                Self::new(self.x - 1, self.y + 1)
            }

            pub fn below(&self) -> Self {
                Self::new(self.x, self.y + 1)
            }

            pub fn below_right(&self) -> Self {
                Self::new(self.x + 1, self.y + 1)
            }

            pub fn cab_distance(&self, other: &Self) -> usize {
                let x = (self.x as isize - other.x as isize).abs() as usize;
                let y = (self.y as isize - other.y as isize).abs() as usize;
                x + y
            }
        })*
    };
}

impl_ints_methods!(isize, i32, i64, usize, u32, u64);

macro_rules! impl_float_methods {
    ($($t:ty),*) => {
        $(impl Point<$t> {
            pub fn euclidean_distance(&self, other: &Self) -> $t {
                let x = self.x - other.x;
                let y = self.y - other.y;
                (x * x + y * y).sqrt()
            }
        })*
    }
}

impl_float_methods!(f64, f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance_between_two_points() {
        let a: Point<f64> = Point::new(0.0, 0.0);
        let b: Point<f64> = Point::new(3.0, 4.0);

        assert_eq!(a.euclidean_distance(&b), 5.0);
    }

    #[test]
    fn test_cab_distance_between_two_points() {
        let a: Point<isize> = Point::new(0, 0);
        let b: Point<isize> = Point::new(3, 4);

        assert_eq!(a.cab_distance(&b), 7);
    }
}
