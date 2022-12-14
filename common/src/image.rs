use std::fs::File;
use std::io::Write;

pub struct PPM {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

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
    fn to_rgb(&self) -> (u8, u8, u8) {
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

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; (3 * width * height) as usize],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        let offset = (y * self.width * 3) + (x * 3);
        let (r, g, b) = color.to_rgb();
        self.data[offset] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}
