use rusqlite::{Connection, params, Result};
use crate::models::todos::{Todo, TodoInput};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("my_todo_app.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
                  id              INTEGER PRIMARY KEY,
                  title           TEXT NOT NULL,
                  completed       BOOLEAN NOT NULL
                  )",
        [],
    )?;

    Ok(conn)
}

pub fn get_todos(conn: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;
    let todos_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;
    
    let todos: Result<Vec<_>, _> = todos_iter.collect();
    todos
}

pub fn create_todo(conn: &Connection, todo_input: &TodoInput) -> Result<Todo> {
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
        params![todo_input.title, false],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Todo {
        id: id as i32,
        title: todo_input.title.clone(),
        completed: false,
    })
}

pub fn get_todo(conn: &Connection, todo_id: i32) -> Result<Todo> {
    conn.query_row(
        "SELECT id, title, completed FROM todos WHERE id = ?1",
        params![todo_id],
        |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        },
    )
}

pub fn update_todo(conn: &Connection, todo_id: i32, todo_input: &TodoInput) -> Result<Todo> {
    conn.query_row(
        "UPDATE todos SET title = ?1 WHERE id = ?2 returning *",
        params![todo_input.title,todo_id],
        |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        },
    )
}

pub fn delete_todo(conn: &Connection, todo_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM todos WHERE id = ?1",
        params![todo_id],
    )?;
    Ok(())
}

pub fn toggle_complete(conn: &Connection, todo_id: i32) -> Result<Todo> {
    conn.query_row(
        "UPDATE todos SET completed = not completed WHERE id = ?1 returning *",
        params![todo_id],
        |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        },
    )
}