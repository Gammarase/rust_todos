use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let connection = Connection::open("src/database/main.db")?;
    
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT,
            deadline TEXT,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
        ",
        []
    )?;
    
    println!("Tables created!");
    Ok(())
}
