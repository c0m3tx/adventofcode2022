pub type Stacks = [Vec<char>; 9];

#[derive(Debug, PartialEq, Eq)]
pub struct MoveCommand {
    pub amount: u64,
    pub from_column: usize,
    pub to_column: usize,
}
impl MoveCommand {
    pub fn new(amount: u64, from_column: usize, to_column: usize) -> Self {
        Self {
            amount,
            from_column,
            to_column,
        }
    }
}

pub trait PrintableStack {
    fn top_elements(&self) -> String;
}

impl PrintableStack for Stacks {
    fn top_elements(&self) -> String {
        self.iter().map(|s| s.last().unwrap_or(&' ')).collect()
    }
}
