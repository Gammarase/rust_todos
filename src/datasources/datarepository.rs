use rusqlite::{Connection, Result, params, Row};
use rusqlite::OptionalExtension;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Option<i64>,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub deadline: Option<String>,
}

pub struct Datasource {
    conn: Connection,
}

impl Datasource {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("src/database/main.db")?;
        
        Ok(Datasource { conn })
    }

    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn register_user(&self, name: &str, password: &str) -> Result<i64> {
        let hashed_password = Self::hash_password(password);
        
        self.conn.execute(
            "INSERT INTO users (name, password) VALUES (?1, ?2)",
            params![name, hashed_password]
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn login(&self, name: &str, password: &str) -> Result<Option<User>> {
        let hashed_password = Self::hash_password(password);
        
        let user = self.conn.query_row(
            "SELECT id, name, password FROM users WHERE name = ?1 AND password = ?2", 
            params![name, hashed_password],
            |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    password: row.get(2)?,
                })
            }
        ).optional()?;

        Ok(user)
    }

    pub fn create_todo(&self, todo: &Todo) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO todos (user_id, name, description, status, deadline) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                todo.user_id, 
                todo.name, 
                todo.description, 
                todo.status, 
                todo.deadline
            ]
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_todo(&self, todo_id: i64) -> Result<Option<Todo>> {
        let todo = self.conn.query_row(
            "SELECT id, user_id, name, description, status, deadline 
             FROM todos WHERE id = ?1", 
            params![todo_id],
            |row| self.row_to_todo(row)
        ).optional()?;

        Ok(todo)
    }

    pub fn get_user_todos(&self, user_id: i64) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, user_id, name, description, status, deadline 
             FROM todos WHERE user_id = ?1"
        )?;

        let todos_iter = stmt.query_map(params![user_id], |row| self.row_to_todo(row))?;
        
        let mut todos = Vec::new();
        for todo in todos_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }

    pub fn update_todo(&self, todo: &Todo) -> Result<()> {
        self.conn.execute(
            "UPDATE todos 
             SET name = ?1, description = ?2, status = ?3, deadline = ?4 
             WHERE id = ?5",
            params![
                todo.name, 
                todo.description, 
                todo.status, 
                todo.deadline, 
                todo.id
            ]
        )?;

        Ok(())
    }

    pub fn delete_todo(&self, todo_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM todos WHERE id = ?1", 
            params![todo_id]
        )?;

        Ok(())
    }

    fn row_to_todo(&self, row: &Row) -> Result<Todo> {
        Ok(Todo {
            id: Some(row.get(0)?),
            user_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            status: row.get(4)?,
            deadline: row.get(5)?,
        })
    }
}

