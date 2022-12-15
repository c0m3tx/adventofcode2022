use super::color::Color;
use image::{ImageBuffer, ImageError, Rgb, RgbImage};

pub struct Image {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: RgbImage::new(width as u32, height as u32),
        }
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        let (r, g, b) = color.to_rgb();
        self.data.put_pixel(x, y, image::Rgb([r, g, b]));
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), ImageError> {
        self.data.save(filename)
    }
}
