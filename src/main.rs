mod cli;
mod database;
mod logger;
mod resources;
mod schemas;
mod server;
mod views;

use std::{error::Error, net::IpAddr};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::make_cli_instance();
    let matches = cli.get_matches();

    // Initialize database connection
    let db = database::connect().await?;

    let (addr, port) = match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            let addr = sub_matches
                .get_one::<String>("bind")
                .unwrap()
                .parse::<IpAddr>()?;
            let port = sub_matches
                .get_one::<String>("port")
                .unwrap()
                .parse::<u16>()?;
            (addr, port)
        }
        _ => unreachable!("Subcommand required"),
    };

    let server = server::Server { db };
    server.serve(addr, port).await?;

    Ok(())
}
