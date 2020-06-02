// The algorithm (Recursive Backtracker):

// Choose the initial cell, mark it as visited and push it to the stack
// While the stack is not empty
//     Pop a cell from the stack and make it a current cell
//     If the current cell has any neighbours which have not been visited
//         Push the current cell to the stack
//         Choose one of the unvisited neighbours
//         Remove the wall between the current cell and the chosen cell
//         Mark the chosen cell as visited and push it to the stack

use image::{Rgb, RgbImage};
use ndarray::prelude::*;
use ndarray::{Array, OwnedRepr};
use rand::prelude::*;
use rand::seq::SliceRandom;

use std::time::Instant;

static WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
static BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);

#[derive(Copy, Clone)]
struct Cell {
    row: usize,
    col: usize,
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Cell { row, col }
    }

    fn above(&self, maze: RgbImage) -> Option<Self> {
        
    }

    fn to_img(&self) -> (u32, u32) {
        (self.row as u32 * 2 + 1, self.col as u32 * 2 + 1)
    }
}

pub fn generate(width: u32, height: u32, of: String, time_it: bool) {
    // For timing it
    let start_time = Instant::now();
    let mut rng = rand::thread_rng();

    // To account for walls
    let mut maze = RgbImage::new(width, height);

    let mut cells = Array::<u8, Ix2>::zeros((width as usize, height as usize).f());

    first = Cell::new(
        rng.gen_range(0, height),
        rng.gen_range(0, width)
    );

    maze.put_pixel(first.col, first.row, WHITE);

    // To work as a LiFo queue
    let mut stack = Vec::new();
    stack.push(first);

    while !stack.is_empty() {
        cur = stack.pop();

        neighbours: Vec<Cell> = vec![];


    }


    if time_it {
        println!("It took {} seconds to generate the maze.", start_time.elapsed().as_secs_f64())
    }
    maze.save(of).expect("Error saving the maze.");
}
