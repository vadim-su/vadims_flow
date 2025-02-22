use std::error::Error;
use surrealdb::engine::any::Any;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn connect() -> Result<Surreal<Db>, Box<dyn Error>> {
    // Create an in-memory database with websocket connection
    let db = Surreal::new::<Mem>(()).await?;

    // For development, we'll use root credentials
    // In production, this should be configured via environment variables
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;

    // Use "test" namespace and database
    db.use_ns("test").use_db("test").await?;

    Ok(db)
}

#[derive(Clone)]
pub struct Database {
    pub connection: Surreal<Any>,
}
