#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

use clap::Parser;

#[derive(Parser)]
struct Args {
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(f) = args.filename {
        println!("{f}");
    }
}
