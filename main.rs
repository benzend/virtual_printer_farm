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
                "INSERT INTO printer (port) VALUE ($1)",
                &[&port]
            )?;
            println!("printer created");
            Ok(())
        },
        _ => panic!("not a valid command")
    }
}

fn setup_db_client() -> Result<Client, Box<dyn std::error::Error>> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let printers_exist_query = client.query("
        SELECT_EXISTS (
            SELECT FROM
                pg_tables
            WHERE
                schemaname = 'public' AND
                tablename = 'printer'
        )
    ", &[])?;

    let mut printers_exist = false;
    for row in printers_exist_query {
        printers_exist = row.get(0);
    }
    if !printers_exist {
        client.batch_execute("
            CREATE TABLE printer (
                id SERIAL PRIMARY KEY,
                port TEXT NOT NULL,
            )
        ")?;
    }

    Ok(client)
}

