#[cfg(any(feature = "png_image", feature = "ppm_image"))]
pub mod image_output;

pub mod matrix_2d;
pub use matrix_2d::Matrix2D;

pub mod point;
pub use point::Point;
