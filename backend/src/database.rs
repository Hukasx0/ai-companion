use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

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
    pub long_term_mem: usize,
    pub short_term_mem: u32,
    pub roleplay: u32,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserData {
    pub id: u32,
    pub name: String,
    pub persona: String,
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
                date TEXT NOT NULL
            )", [],
        ).unwrap();
        con.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                persona TEXT NOT NULL
            )", [],
        ).unwrap();
        con.execute(
            "CREATE TABLE IF NOT EXISTS companion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                persona TEXT NOT NULL,
                first_message TEXT NOT NULL,
                long_term_mem INTEGER NOT NULL,
                short_term_mem INTEGER NOT NULL,
                roleplay INTEGER NOT NULL,
                avatar_path STRING NOT NULL
            )", [],
        ).unwrap();
        if Database::is_table_empty("companion", &con) {
            con.execute(
                "INSERT INTO companion (id, name, persona, first_message, long_term_mem, short_term_mem, roleplay, avatar_path) VALUES (NULL, \"Assistant\", \"Assistant is an artificial intelligence model designed to help the user\", \"hello user, how can i help you?\", 2, 5, 1, \"/assets/default.jpg\")", []
            );
        }
        if Database::is_table_empty("user", &con) {
            con.execute(
                "INSERT INTO user (id, name, persona) VALUES (NULL, \"user\", \"user chatting with artificial intelligence\")", []
            );
        }
        if Database::is_table_empty("messages", &con) {
            let local: DateTime<Local> = Local::now();
            let formatted_date = local.format("%A %d.%m.%Y %H:%M").to_string();
            let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0)).unwrap();
            return con.execute(
                &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", ?1, \"{}\")", formatted_date), &[&first_message]
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

    pub fn get_x_msgs(msgs_limit: u32) -> Vec<Message> {
        let con = Connection::open("companion.db").unwrap();
        let mut stmt = con.prepare(&format!("SELECT id, ai, text, date FROM messages ORDER BY id DESC LIMIT {}", msgs_limit)).unwrap();
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
                long_term_mem: row.get(4)?,
                short_term_mem: row.get(5)?,
                roleplay: row.get(6)?,
                avatar_path: row.get(7)?,
            })
        }).unwrap();
        let mut result: CompanionData = Default::default();
        for companion in companion_data {
            result = companion.unwrap();
         }
        result
    }

    pub fn get_user_data() -> UserData {
        let con = Connection::open("companion.db").unwrap();
        let mut stmt = con.prepare("SELECT * FROM user LIMIT 1").unwrap();
        let user_data = stmt
        .query_map([], |row| {
            Ok(UserData {
                id: row.get(0)?,
                name: row.get(1)?,
                persona: row.get(2)?,
            })
        }).unwrap();
        let mut result: UserData = Default::default();
        for user in user_data {
            result = user.unwrap();
         }
        result
    }

    pub fn add_message(text: &str, is_ai: bool) {
        let con = Connection::open("companion.db").unwrap();
        let ai = &is_ai.to_string();
        let local: DateTime<Local> = Local::now();
        let formatted_date = &local.format("%A %d.%m.%Y %H:%M").to_string();
        con.execute("INSERT INTO messages (id, ai, text, date) VALUES (NULL, ?1, ?2, ?3)", &[&ai.as_str(), &text, &formatted_date.as_str()]).unwrap();
    }

    pub fn clear_messages() {
        let con = Connection::open("companion.db").unwrap();
        con.execute("DELETE FROM messages", []).unwrap();
        let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0)).unwrap();
        let local: DateTime<Local> = Local::now();
        let formatted_date = &local.format("%A %d.%m.%Y %H:%M").to_string();
        con.execute(
            &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", ?1, \"{}\")", formatted_date), &[&first_message]
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

    pub fn change_companion(name: &str, persona: &str, first_message: &str, long_term_mem: u32, short_term_mem: u32, roleplay: bool) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET name=\"{}\", persona=\"{}\", first_message=\"{}\", long_term_mem={}, short_term_mem={}, roleplay={}", name, persona, first_message, long_term_mem, short_term_mem, roleplay), []).unwrap();
    }

    pub fn change_companion_avatar(path: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET avatar_path=\"{}\"", path), []).unwrap();
    }

    pub fn import_companion(name: &str, persona: &str, first_message: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute("UPDATE companion SET name=?1, persona=?2, first_message=?3", &[&name, &persona, &first_message]).unwrap();
    }

    pub fn rm_message(id: u32) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("DELETE FROM messages WHERE id={}", id), []).unwrap();
    }

    pub fn change_username(name: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE user SET name=\"{}\"", name), []).unwrap();
    }

    pub fn change_user_persona(persona: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE user SET persona=\"{}\"", persona), []).unwrap();
    }

    pub fn change_user(name: &str, persona: &str) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE user SET name=\"{}\", persona=\"{}\"", name, persona), []).unwrap();
    }

    pub fn change_short_term_memory(limit: u32) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET short_term_mem={}", limit), []).unwrap();
    }

    pub fn change_long_term_memory(limit: u32) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET long_term_mem={}", limit), []).unwrap();
    }

    pub fn disable_enable_roleplay(op: bool) {
        let con = Connection::open("companion.db").unwrap();
        con.execute(&format!("UPDATE companion SET roleplay={}", op), []).unwrap();
    }
}
