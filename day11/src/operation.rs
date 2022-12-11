#[derive(Debug, PartialEq)]
pub enum Item {
    Current,
    Constant(isize),
}

impl From<&str> for Item {
    fn from(item: &str) -> Self {
        if item == "old" {
            Self::Current
        } else {
            Self::Constant(item.parse().expect("Invalid value"))
        }
    }
}

impl Item {
    fn or_current(&self, current: isize) -> isize {
        match &self {
            Self::Current => current,
            Self::Constant(c) => *c,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add(Item, Item),
    Mult(Item, Item),
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        if op.contains("*") {
            let (left, right) = op.split_once(" * ").unwrap();
            Operation::Mult(left.into(), right.into())
        } else {
            let (left, right) = op.split_once(" + ").unwrap();
            Operation::Add(left.into(), right.into())
        }
    }
}

impl Operation {
    pub fn apply(&self, current: isize) -> isize {
        match &self {
            Self::Add(left, right) => {
                let left = left.or_current(current);
                let right = right.or_current(current);
                left + right
            }
            Self::Mult(left, right) => {
                let left = left.or_current(current);
                let right = right.or_current(current);
                left * right
            }
        }
    }
}
