use std::path::PathBuf;

use clap::{Parser, Subcommand};
mod benchmark;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        name = "benchmark", 
        visible_aliases = ["bench", "bm"]
    )]
    Benchmark {
        #[arg(default_value = "localhost")]
        host: String,

        #[arg(short, long, default_value_t = 555)]
        modem_port: u16,

        #[arg(short, long, default_value_t = 5555)]
        program_port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match cli.command {
        Commands::Benchmark {
            host,
            modem_port,
            program_port,
        } => {
            println!("Host: {host}, modem_port: {modem_port}, program_port: {program_port}");
            let bench = move || benchmark::benchmark(host.clone(), modem_port, program_port);
            benchmark::bench_with_criterion(bench);
        }
    }

    // Continued program logic goes here...
}
