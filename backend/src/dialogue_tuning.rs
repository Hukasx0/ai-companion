use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct Dialogue {
    pub user_msg: String,
    pub ai_msg: String,
}

pub struct DialogueTuning {}

impl DialogueTuning {
    pub fn create() -> Result<usize, Error> {
        let con = Connection::open("companion.db")?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS dialogues (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_msg TEXT NOT NULL,
                ai_msg TEXT NOT NULL
            )", [],
        )
    }

    pub fn add_dialogue(user_msg: &str, ai_msg: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(
            "INSERT INTO dialogues (user_msg, ai_msg) VALUES (?1, ?2)",
            &[user_msg, ai_msg],
        )?;
        Ok(())
    }

    pub fn get_random_dialogue() -> Result<Dialogue, Error> {
        let con = Connection::open("companion.db")?;

        let query = "SELECT user_msg, ai_msg FROM dialogues WHERE id = (SELECT id FROM dialogues ORDER BY RANDOM() LIMIT 1);";

        if let Some(row) = con.query_row(query, [], |row| {
            let user_msg: String = row.get(0)?;
            let ai_msg: String = row.get(1)?;
            Ok(Dialogue { user_msg, ai_msg })
        }).ok() {
            Ok(row)
        } else {
            Err(Error::QueryReturnedNoRows)
        }
    }

    pub fn clear_dialogues() -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute("DELETE FROM dialogues", [])?;
        Ok(())
    }
}
