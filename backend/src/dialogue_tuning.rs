use rusqlite::{Connection, Error, Result};

pub struct Dialogue {
    pub user_msg: String,
    pub ai_msg: String,
}

pub struct DialogueTuning { }

impl DialogueTuning {
    pub fn create() -> Result<usize, Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute("CREATE TABLE IF NOT EXISTS dialogue_tuning (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_msg TEXT,
            ai_msg TEXT
        )", [])
    }

    pub fn insert(user_msg: &str, ai_msg: &str) -> Result<usize, Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute("INSERT INTO dialogue_tuning (user_msg, ai_msg) VALUES (?1, ?2)", [user_msg, ai_msg])
    }

    pub fn get_random_dialogue() -> Result<Dialogue, Error> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT user_msg, ai_msg FROM dialogue_tuning ORDER BY RANDOM() LIMIT 1")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            let user_msg: String = row.get(0)?;
            let ai_msg: String = row.get(1)?;
            Ok(Dialogue { user_msg, ai_msg })
        }
        else {
            Err(Error::QueryReturnedNoRows)
        }
    }

    pub fn clear_dialogues() -> Result<usize, Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute("DELETE FROM dialogue_tuning", [])
    }
}
