use clap::{Parser, Subcommand};
use commands::run;

mod commands;
mod api;

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Initialize a new rask entry
    Run(run::Arguments),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "Rask - The universal way of running tasks", long_about = None, propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    command: Command
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let Arguments { command } = Arguments::parse();

    let result = match command {
        Command::Run(ref arguments) => { run::execute(&arguments) },
    };

    return match result.await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}
