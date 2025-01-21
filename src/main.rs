use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mechanic", version = "0.1.0", author = "Ben Dowling")]
struct MechanicCLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "make:model")]
    MakeModel {
        name: String,
    },
}

fn main() {
    let cli: MechanicCLI = MechanicCLI::parse();

    match cli.command {
        Commands::MakeModel {

        }
    }
}
