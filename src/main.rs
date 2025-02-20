mod logger;
mod resources;
mod server;
mod views;

use std::{error::Error, net::Ipv4Addr};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = server::Server {};
    server
        .serve(Ipv4Addr::UNSPECIFIED.to_string(), 8080)
        .await?;
    return Ok(());
}
