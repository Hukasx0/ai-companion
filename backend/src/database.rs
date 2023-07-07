use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub ai: String,
    pub text: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CompanionData {
    pub id: u32,
    pub name: String,
    pub persona: String,
    pub first_message: String,
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
        con.execute(
            "CREATE TABLE IF NOT EXISTS companion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                persona TEXT NOT NULL,
                first_message TEXT NOT NULL
            )", [],
        ).unwrap();
        if Database::is_table_empty("companion", &con) {
            con.execute(
                "INSERT INTO companion (id, name, persona, first_message) VALUES (NULL, \"Assistant\", \"Assistant is an artificial intelligence model designed to help the user\", \"hello user, how can i help you?\")", []
            );
        }
        if Database::is_table_empty("messages", &con) {
            let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0)).unwrap();
            return con.execute(
                &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", \"{}\", date('now'))", first_message), []
            );
        } else {
            return Ok(0);
        }
    }

    pub fn is_table_empty(table_name: &str, con: &Connection) -> bool {
        let count: i64 = con.query_row(&format!("SELECT COUNT(*) FROM {}",table_name), [], |row| row.get(0)).unwrap();
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

    pub fn get_companion_data() -> CompanionData {
        let con = Connection::open("companion.db").unwrap();
        let mut stmt = con.prepare("SELECT * FROM companion LIMIT 1").unwrap();
        let companion_data = stmt
        .query_map([], |row| {
            Ok(CompanionData {
                id: row.get(0)?,
                name: row.get(1)?,
                persona: row.get(2)?,
                first_message: row.get(3)?,
            })
        }).unwrap();
        let mut result: CompanionData = Default::default();
        for companion in companion_data {
            result = companion.unwrap();
         }
        result
    }

    pub fn add_message(text: &str, is_ai: bool) {
        let con = Connection::open("companion.db").unwrap();
        let ai = &is_ai.to_string();
        con.execute("INSERT INTO messages (id, ai, text, date) VALUES (NULL, ?1, ?2, date('now'))", &[&ai.as_str(), &text]).unwrap();
    }

    pub fn clear_messages() {
        let con = Connection::open("companion.db").unwrap();
        con.execute("DELETE FROM messages", []).unwrap();
        let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0)).unwrap();
        con.execute(
            &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", \"{}\", date('now'))", first_message), []
        );
    }

    pub fn change_first_message(first_message: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET first_message=\"{}\"", first_message), []).unwrap();
    }

    pub fn change_companion_name(name: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET name=\"{}\"", name), []).unwrap();
    }

    pub fn change_companion_persona(persona: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET persona=\"{}\"", persona), []).unwrap();
    }

    pub fn change_companion(name: &str, persona: &str, first_message: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET name=\"{}\", persona=\"{}\", first_message=\"{}\"", name, persona, first_message), []).unwrap();
    }

    pub fn rm_message(id: u32) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("DELETE FROM messages WHERE id={}", id), []).unwrap();
    }
}
