use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    TrackFile {
        #[arg(short, long)]
        filename: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            println!("Initializing directory...");
        }
        Command::TrackFile { filename } => {
            println!("Tracking: {}", filename);
        }
    }
}
