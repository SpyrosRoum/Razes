// The algorithm (Recursive Backtracker):

// Choose the initial cell, mark it as visited and push it to the stack
// While the stack is not empty
//     Pop a cell from the stack and make it the current cell
//     If the current cell has any neighbours which have not been visited
//         Push the current cell to the stack
//         Choose one of the unvisited neighbours
//         Remove the wall between the current cell and the chosen cell
//         Mark the chosen cell as visited and push it to the stack

use image::{ImageBuffer, Rgb, RgbImage};
use rand::prelude::*;

static WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);

fn img_idx(x: usize, y: usize) -> (u32, u32) {
    // grid index to image index
    ((x * 2 + 1) as u32, (y * 2 + 1) as u32)
}

fn open_img(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, grid_x: usize, grid_y: usize) {
    let (img_x, img_y) = img_idx(grid_x, grid_y);
    img.put_pixel(img_x, img_y, WHITE);
}

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
    wall: bool,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell { x, y, wall: true }
    }

    fn open(&mut self, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        self.wall = false;
        open_img(img, self.x, self.y);
    }
}

#[derive(Clone)]
pub struct Generator {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Generator {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let cell = Cell::new(x, y);
                row.push(cell);
            }
            grid.push(row);
        }

        Generator {
            grid,
            width,
            height,
        }
    }

    pub fn generate(&mut self) -> RgbImage {
        let mut rng = rand::thread_rng();

        // * 2 + 1 to account for walls
        let mut maze = ImageBuffer::new((self.width * 2 + 1) as u32, (self.height * 2 + 1) as u32);

        let mut stack: Vec<(usize, usize)> = Vec::new();

        let x = rng.gen_range(0, self.width - 1);

        let first = &mut self.grid[0][x];
        first.open(&mut maze);
        stack.push((first.x, first.y));

        // Open entry, so one above first
        maze.put_pixel((first.x * 2 + 1) as u32, 0u32, WHITE);
        // Exit
        maze.put_pixel((self.width * 2 - 1) as u32, (self.height * 2) as u32, WHITE);


        while !stack.is_empty() {
            let (cur_x, cur_y) = stack.pop().unwrap();
            let neighbours = self.get_close_neighbours(cur_x, cur_y);

            if !neighbours.is_empty() {
                let rand_idx = rng.gen_range(0, neighbours.len());
                let (next_x, next_y) = neighbours[rand_idx];

                // remove wall
                // This works because `next_cell.row + row + 1` is basically the average of the pixels position,
                // Which means between the two!
                // https://canary.discordapp.com/channels/434207294184620043/434352882507317250/732979707754315844
                maze.put_pixel((next_x + cur_x + 1) as u32, (next_y + cur_y + 1) as u32, WHITE);

                self.grid[next_y][next_x].open(&mut maze);

                stack.push((cur_x, cur_y));
                stack.push((next_x, next_y));
            }
        }

        maze
    }

    fn get_close_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        // Find all the closed neighbours
        let mut neighbours = Vec::new();

        if y > 0 {
            // above
            if self.grid[y - 1][x].wall {
                neighbours.push((x, y - 1));
            }
        }
        if y < self.height - 1 {
            // under
            if self.grid[y + 1][x].wall {
                neighbours.push((x, y + 1));
            }
        }
        if x > 0 {
            // left
            if self.grid[y][x - 1].wall {
                neighbours.push((x - 1, y));
            }
        }
        if x < self.width - 1 {
            // right
            if self.grid[y][x + 1].wall {
                neighbours.push((x + 1, y));
            }
        }

        neighbours
    }
}
