use clap::{Parser, Subcommand};
use postgres::{Client, NoTls};


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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut client = match setup_db_client() {
        Ok(client) => client,
        Err(e) => panic!("{}", e.to_string())
    };

    let port = match &cli.port {
        Some(p) => p,
        None => &(3000 as i64)
    };

    match &cli.command {
        Some(Commands::Create) => {
            client.execute(
                "INSERT INTO printer (port) VALUES ($1)",
                &[&port.to_string()]
            )?;
            println!("printer created");
            Ok(())
        },
        _ => panic!("not a valid command")
    }
}

fn setup_db_client() -> Result<Client, Box<dyn std::error::Error>> {
    let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS printer (
            id    SERIAL PRIMARY KEY,
            port  TEXT NOT NULL
        )
    ")?;

    Ok(client)
}

// * The reference python script we will need for creating virtual printers
// popen = subprocess.Popen([
//     "./OctoPrint/venv/bin/activate", "-c",
//     "octoprint", "serve", "--port='{}'".format(item.port)
// ], stdout=subprocess.PIPE, shell=True)
