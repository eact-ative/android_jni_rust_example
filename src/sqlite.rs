use std::result;

use rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
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

pub struct ResourceManager {
    conn: Connection,
}

impl ResourceManager {
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let table_exist = check_table_exist(&conn, "resource")?;
        if !table_exist {
            create_tables(&conn)?;
        }
        Ok(ResourceManager { conn: conn })
    }

    fn add_entity(&self, entity: &ResouceEntity) -> Result<()> {
        self.conn.execute("
        INSERT INTO resource (url, path, hash_code, cache_ctrl) VALUES (?1, ?2, ?3, ?4)
        ", (&entity.url, &entity.path, &entity.hashCode, &entity.cacheCtrl))?;
        Ok(())
    }

    fn delete_entity(&self, id: u32) -> Result<()> {
        self.conn.execute("DELETE FROM resource WHERE id = ?1 ", [id])?;
        Ok(())
    }

    fn get_entity(&self, url: &str) -> Result<Option<ResouceEntity>> {
        let mut stmt = self.conn.prepare("
        SELECT id, url, path, hash_code, cache_ctrl FROM resource where url = ?
        ")?;
        let mut resource_iter = stmt.query_map([url], |row| {
            Ok(ResouceEntity {
                id: row.get(0)?,
                url: row.get(1)?,
                path: row.get(2)?,
                hashCode: row.get(3)?,
                cacheCtrl: row.get(4)?,
            })
        })?;
        let mut row = resource_iter.next();
        match row {
            Some(r) => match r {
                Ok(entity) => Ok(Some(entity)),
                Err(e) => Err(StorageError::Db { sourece: (e) }),
            },
            None => Ok(None)
        }
    }
}

fn check_table_exist(conn: &Connection, table: &str) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let mut rows = stmt.query([table])?;
    let row = rows.next()?;
    match row {
        Some(r) => Ok(true),
        None => Ok(false),
    }
}

fn create_tables(conn: &Connection) -> Result<()> {
    let sql = "CREATE TABLE resource (
        id          INTEGER PRIMARY KEY,
        url         TEXT NOT NULL,
        path        TEXT NOT NULL,
        hash_code   TEXT NOT NULL,
        cache_ctrl  TEXT NOT NULL,
    ";
    let mut stms = conn.execute(sql, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::path::Path;


    use rusqlite::{Connection, Error};

    use crate::sqlite::ResourceManager;

    use super::{check_table_exist, Person, Result};

    #[test]
    fn test_sqllite() -> Result<()> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
            (),
        )?;

        let me = Person {
            id: 0,
            name: "steven".to_string(),
            data: None,
        };

        println!("table exist {:?}", check_table_exist(&conn, "talbe_name"));

        

        conn.execute(
            "INSERT INTO person (name, data) VALUES (?1, ?2)",
            (&me.name, &me.data),
        );

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

    #[test]
    fn test_open_no_exist_db_file() -> Result<()> {
        let binding = std::env::current_dir().unwrap();
        let current_dir = binding.to_str().unwrap();
        let db_name = "test.sqlite";
        let db_path = format!("{}/{}", current_dir, db_name);
        if Path::new(&db_path).exists() {
            std::fs::remove_file(&db_path);
        }
        let manager = ResourceManager::new(db_path.as_str())?;
        Ok(())
    }
    
}
