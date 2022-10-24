use rusqlite::{Connection};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Error};

    use super::Person;

    #[test]
    fn test_sqllite() -> Result<(), Error> {
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