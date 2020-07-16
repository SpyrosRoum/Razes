use image::{ImageBuffer, ImageFormat, Rgb, RgbImage, self};

pub struct Solver {
    in_file: String,
}

impl Solver {
    pub fn new(in_file: String) -> Self {
        Solver {
            in_file,
        }
    }

    pub fn solve(&self) -> RgbImage {
    }
}
