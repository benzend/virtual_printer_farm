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
    List,
}

struct Ctx {
    db_client: Client,
    cli: Cli,
}

impl Ctx {
    fn new(db_client: Client, cli: Cli) -> Ctx {
        Ctx { db_client, cli }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = match setup_db_client() {
        Ok(client) => client,
        Err(e) => panic!("{}", e.to_string())
    };

    let mut ctx = Ctx::new(client, cli);

    let port = match &ctx.cli.port {
        Some(p) => *p,
        None => 3000 as i64
    };

    match &ctx.cli.command {
        Some(Commands::Create) => {
            handle_create_printer(&port, &mut ctx)
        },
        Some(Commands::List) => {
            handle_list_printers(&mut ctx)
        },
        _ => panic!("not a valid command")
    }
}

fn handle_list_printers(ctx: &mut Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let query = ctx.db_client.query("SELECT * FROM printer", &[])?;

    for row in query {
        let data: String = row.get(0);
        println!("{}", data);
    }

    Ok(())
}

fn handle_create_printer(port: &i64, ctx: &mut Ctx) -> Result<(), Box<dyn std::error::Error>> {
    ctx.db_client.execute(
        "INSERT INTO printer (port) VALUE ($1)",
        &[port]
    )?;
    println!("printer created");
    Ok(())
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
