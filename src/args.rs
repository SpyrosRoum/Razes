use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(short, long, help = "Time the operation")]
    pub time_it: bool,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Gen {
        #[structopt(
            short,
            long,
            default_value = "32",
            help = "The width of the maze generated"
        )]
        width: u32,

        #[structopt(
            short,
            long,
            default_value = "32",
            help = "The height of the maze generated"
        )]
        height: u32,

        #[structopt(
            short,
            long,
            default_value = "generated.png",
            help = "The generated maze"
        )]
        output_file: String,
    },

    Solve {
        #[structopt(short, long, help = "The maze to solve")]
        input_file: String,

        #[structopt(short, long, default_value = "solved.png", help = "The solved maze")]
        output_file: String,
    },
}

impl Args {
    pub fn collect() -> Args {
        Args::from_args()
    }
}
