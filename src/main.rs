use clap::Parser;
use log::info;
use std::{
    error::Error,
    io::{self, Write},
    process,
};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "NUM_UUIDS", default_value = "1")]
    num_uuids: u16,
}

fn main() {
    env_logger::init();

    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_app() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    info!("Generating {} UUIDs", args.num_uuids);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for _ in 0..args.num_uuids {
        writeln!(handle, "{}", Uuid::new_v4().as_hyphenated())?;
    }

    Ok(())
}
