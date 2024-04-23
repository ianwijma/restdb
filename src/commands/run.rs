use std::fmt::Debug;
use clap::Args;
use crate::api::connection::Connection;

#[derive(Args, Debug, Clone)]
pub struct Arguments {
    #[arg(long, help = "The SQL connection string")]
    connection_string: String,
}

pub async fn execute (arguments: &Arguments) -> Result<(), String> {
    let Arguments { connection_string } = arguments;

    let connection = Connection::new(connection_string).await.unwrap();

    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct Product {
        id: i64,
        slug: String,
        name: String,
    }

    let rows: Vec<Product> = sqlx::query_as("SELECT * from product")
        .fetch_all(&connection.pool).await.unwrap();

    eprintln!("rows = {:?}", rows);



    Ok(())
}