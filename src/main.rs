mod args;
mod generator;
mod solver;

use args::{Args, Command};

fn main() {
    let args = Args::collect();

    match args.cmd {
        Command::Gen {width, height, output_file} => generator::generate(width, height, output_file, args.time_it),
        Command::Solve {input_file, output_file} => solver::solve(input_file, output_file, args.time_it)
    }
}
