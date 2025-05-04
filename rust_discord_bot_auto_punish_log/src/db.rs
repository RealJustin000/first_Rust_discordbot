use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("bot_data.sqlite3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS warnings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            moderator_id TEXT NOT NULL,
            reason TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}