use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    port: Option<i64>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create) => {
            println!("printer created");
        },
        _ => panic!("faulty command")
    }
}
