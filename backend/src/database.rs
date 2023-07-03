use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub ai: String,
    pub text: String,
    pub date: String,
}

pub struct Database {}

impl Database {
    pub fn create() -> Result<usize> {
        let con = Connection::open("companion.db")?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ai BOOLEAN NOT NULL,
                text TEXT NOT NULL,
                date DATE NOT NULL
            )", [],
        ).unwrap();
        if Database::is_table_empty(&con) {
            return con.execute(
                "INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", \"hello user, how can i help you?\", date('now'))", []
            );
        } else {
            return Ok(0);
        }
    }

    pub fn is_table_empty(con: &Connection) -> bool {
        let count: i64 = con.query_row("SELECT COUNT(*) FROM messages", [], |row| row.get(0)).unwrap();
        return count == 0;
    }

    pub fn get_messages() -> Vec<Message> {
        let con = Connection::open("companion.db").unwrap();
        let mut stmt = con.prepare("SELECT id, ai, text, date FROM messages").unwrap();
        let message_rows = stmt.query_map([], |row| {
            Ok(Message {
                id: row.get(0).unwrap(),
                ai: row.get(1).unwrap(),
                text: row.get(2).unwrap(),
                date: row.get(3).unwrap(),
            })
        }).unwrap();
        let mut messages: Vec<Message> = Vec::new();
        for msgs in message_rows {
           messages.push(msgs.unwrap());
        }
        messages
    }

    pub fn get_five_msgs() -> Vec<Message> {
        let con = Connection::open("companion.db").unwrap();
        let mut stmt = con.prepare("SELECT id, ai, text, date FROM messages ORDER BY id DESC LIMIT 5").unwrap();
        let message_rows = stmt
        .query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                text: row.get(2)?,
                date: row.get(3)?,
            })
        })
        .unwrap();
        let mut messages: Vec<Message> = Vec::new();
        for msgs in message_rows {
           messages.push(msgs.unwrap());
        }
        messages.into_iter().rev().collect()
    }

    pub fn add_message(text: &str, is_ai: bool) {
        let con = Connection::open("companion.db").unwrap();
        let ai = &is_ai.to_string();
        con.execute("INSERT INTO messages (id, ai, text, date) VALUES (NULL, ?1, ?2, date('now'))", &[&ai.as_str(), &text]).unwrap();
    }

}
