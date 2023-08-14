use std::{
    fs::{self, OpenOptions},
    io::ErrorKind,
};

use tracing::{info, warn};

use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
    Error as SqlxError, Row,
};

#[derive(Clone)]
pub struct Database {
    conn: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, SqlxError> {
        // Path taken from https://docs.rs/sqlx/0.6.3/sqlx/sqlite/struct.SqliteConnectOptions.html
        // Creates the database file if it doesn't already exist.
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("actt.db")
        {
            Err(e) if e.kind() != ErrorKind::AlreadyExists => {
                return Err(SqlxError::Io(e));
            }
            _ => {}
        }

        let db_path = "sqlite://actt.db";
        let conn = SqlitePoolOptions::new().connect(db_path).await?;
        let db_creation_query = fs::read_to_string("./actt.sql")?;
        query(db_creation_query.as_str()).execute(&conn).await?;

        return Ok(Self { conn });
    }
}
