pub enum Color {
    Red,
    Blue,
    Green,
    Gray,
    Black,
    White,
    Magenta,
    Yellow,
    Cyan,
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Red => (255, 0, 0),
            Self::Blue => (0, 0, 255),
            Self::Green => (0, 255, 0),
            Self::Gray => (128, 128, 128),
            Self::Black => (0, 0, 0),
            Self::White => (255, 255, 255),
            Self::Magenta => (255, 0, 255),
            Self::Yellow => (255, 255, 0),
            Self::Cyan => (0, 255, 255),
        }
    }
}
