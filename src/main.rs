mod args;
mod generator;
mod solver;

use args::{Args, Command};
use std::time::Instant;

use generator::Generator;
use solver::Solver;
use image::RgbImage;


fn time_it<T, F: FnMut(T) -> RgbImage>(fun: F, for_: T) -> RgbImage {
    let start_time = Instant::now();

    let img = fun(for_);

    println!(
        "It took {} seconds.",
        start_time.elapsed().as_secs_f64()
    );

    img
}

fn main() {
    let args = Args::collect();

    match args.cmd {
        Command::Gen {
            width,
            height,
            output_file,
        } => {
            let mut gen = Generator::new(width as usize, height as usize);

            let img = if args.time_it {
                    time_it(Generator::generate, &mut gen)
            } else {
                gen.generate()
            };


            println!("Saving as {}..", output_file);
            img.save(output_file).expect("Error saving the maze.");
        }
        Command::Solve {
            input_file,
            output_file,
        } => {
            let solver = Solver::new(input_file);

            let img = if args.time_it {
                time_it(Solver::solve, solver)
            } else {
                solver.solve()
            };

            println!("Saving as {}..", output_file);
            img.save(output_file).expect("Error saving the maze.");
        }
    }
}
