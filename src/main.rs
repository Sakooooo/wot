use clap::{Parser, Subcommand};
mod project;

#[derive(Debug, Subcommand)]
enum Commands {
    Init { directory: Option<String> },
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory } => todo!(),
    };
}
