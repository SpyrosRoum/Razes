mod args;
mod generator;
mod solver;

use args::{Args, Command};
use std::time::Instant;

use generator::Generator;

fn main() {
    let args = Args::collect();

    match args.cmd {
        Command::Gen {
            width,
            height,
            output_file,
        } => {
            let start_time = Instant::now();

            // , output_file, args.time_it
            let mut gen = Generator::new(width as usize, height as usize);
            let img = gen.generate();
            if args.time_it {
                println!(
                    "It took {} seconds to generate the maze.",
                    start_time.elapsed().as_secs_f64()
                );
            }
            println!("Saving as {}..", output_file);
            img.save(output_file).expect("Error saving the maze.");
        }
        Command::Solve {
            input_file,
            output_file,
        } => solver::solve(input_file, output_file, args.time_it),
    }
}
