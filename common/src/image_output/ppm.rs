use std::fs::File;
use std::io::Write;

use super::color::Color;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (3 * width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        let offset = ((y * self.width * 3) + (x * 3)) as usize;
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
