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
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory } => project::init(directory),
    };
}
