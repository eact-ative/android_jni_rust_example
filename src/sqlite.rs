use std::result;

use rusqlite::{Connection};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[derive(Debug)]
struct ResouceEntity {
    id: u32,
    url: String,
    path: String,
    hashCode: String,
    cacheCtrl: String,
}

#[derive(Debug)]
pub struct ResourceConfig {
    db_file_path: String, // e.g. /etc/app/mydb.db 
    cache_path: String,
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("db disconnected")]
    DISCONNECT(String),
    #[error("db fail")]
    Db {
        #[from]
        sourece: rusqlite::Error,
    },
}

pub type Result<T, E = StorageError> = result::Result<T, E>;

fn check_table_exist(conn: &Connection, table: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let mut rows = stmt.query([table])?;
    let row = rows.next()?;
    match row {
        Some(r) => Ok(true),
        None => Ok(false)
    }
}

fn create_table(conn: &Connection) ->


#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Error};

    use super::{Person, check_table_exist};

    #[test]
    fn test_sqllite() -> Result<()> {
        let conn = Connection::open_in_memory()?;

        conn.execute("CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )", ())?;

        let me = Person {
            id: 0,
            name: "steven".to_string(),
            data: None,
        };

        println!("table exist {:?}", check_table_exist(&conn, "talbe_name"));
        

        conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data));

        let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;

        for person in person_iter {
            print!("Found person {:?}", person.unwrap());
        }
        Ok(())

    }
}