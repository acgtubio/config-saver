mod commands;

use clap::{Parser, Subcommand};
use commands::{init::init_handler, reset::reset_handler, track::track_file};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, default_value = "")]
    mode: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    Config,
    Init,
    Track {
        #[arg(short, long)]
        filename: String,
    },
    Reset,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Config => {
            print!("showing config...\n");
        }
        Command::Init => {
            init_handler(args.mode);
        }
        Command::Track { filename } => {
            track_file(filename);
        }
        Command::Reset => {
            reset_handler();
        }
    }
}
