// The algorithm (Recursive Backtracker):

// Close all cells
// Choose starting cell and open it. This is the current cell
// Pick a cell adjacent to the current cell that hasn’t been visited and open it. It becomes the current cell
// Repeat 2 until no adjacent wall can be selected
// The previous cell becomes the current cell. If this cell is the starting cell, then we are done. Else go to 2
use image::{RgbImage, Rgb};
use rand::prelude::*;
use ndarray::prelude::*;
use ndarray::Array;


pub fn generate(width: u32, height: u32, of: String, time_id: bool) {

}
