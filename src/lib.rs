use console::style;
use rusqlite::{named_params, Connection, Error};

pub struct TodoList {
    db: Connection,
}

#[derive(Debug)]
struct TodoItem {
    todo_item: String,
    item_completed: bool,
    item_removed: bool,
}

impl TodoList {
    pub fn new() -> Result<TodoList, Error> {
        let db = Connection::open("./todo_db")?;
        Ok(TodoList { db })
    }

    pub fn init_table(&self) -> Result<(), Error> {
        self.db.execute(
            "CREATE TABLE IF NOT EXISTS todo (
                todo_item TEXT NOT NULL,
                item_completed BOOLEAN DEFAULT FALSE NOT NULL,
                item_removed   BOOLEAN DEFAULT FALSE NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add(&self, todo_items: &Vec<String>) -> Result<(), Error> {
        for todo_item in todo_items {
            self.db
                .execute("INSERT INTO todo (todo_item) VALUES (?1)", [todo_item])?;
        }
        Ok(())
    }

    pub fn remove(&self, todo_item_indexes: &Vec<u64>) -> Result<(), Error> {
        for todo_item_index in todo_item_indexes {
            self.db.execute(
                "UPDATE todo SET item_remove = :item_removed WHERE rowid = :rowid",
                named_params! { ":item_remove": true, ":rowid": todo_item_index },
            )?;
        }
        Ok(())
    }

    pub fn edit(&self, todo_item_index: &u64, new_todo_item: &String) -> Result<(), Error> {
        self.db.execute(
            "UPDATE todo SET todo_item = :todo_item WHERE rowid = :rowid",
            named_params! { ":todo_item": new_todo_item, ":rowid": todo_item_index },
        )?;
        Ok(())
    }

    pub fn done(&self, todo_item_indexes: &Vec<u64>) -> Result<(), Error> {
        for todo_item_index in todo_item_indexes {
            self.db.execute(
                "UPDATE todo SET item_completed = :item_completed WHERE rowid = :rowid",
                named_params! { ":item_completed": true, ":rowid": todo_item_index },
            )?;
        }
        Ok(())
    }

    pub fn list(&self) -> Result<(), Error> {
        let mut select_statement = self
            .db
            .prepare("SELECT todo_item, item_completed, item_removed FROM todo")?;
        let db_items_iter = select_statement.query_map([], |row| {
            Ok(TodoItem {
                todo_item: row.get(0)?,
                item_completed: row.get(1)?,
                item_removed: row.get(2)?,
            })
        })?;
        for (index, db_item_res) in db_items_iter.enumerate() {
            let TodoItem {
                mut todo_item,
                item_completed,
                item_removed,
            } = db_item_res?;
            if item_removed {
                continue;
            }
            if item_completed {
                todo_item = style(todo_item).strikethrough().to_string();
            }
            println!("{} {}", index + 1, todo_item);
        }
        Ok(())
    }
}
