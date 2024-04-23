use sqlx::{AnyPool};
use sqlx::any::install_default_drivers;
use sqlx::pool::PoolOptions;

#[derive(Debug)]
pub struct Connection {
    pub pool: AnyPool,
}

impl Connection {
    pub async fn new(connection_string: &String) -> Result<Connection, String> {
        install_default_drivers();

        let pool = PoolOptions::new()
            .max_connections(10)
            .connect(connection_string.as_str()).await;

        return match pool {
            Ok(pool) => Ok(Connection { pool }),
            Err(err) => Err(err.to_string())
        }
    }
}