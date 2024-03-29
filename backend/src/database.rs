use rusqlite::{Connection, Result, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub ai: bool,
    pub content: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub ai: bool,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Companion {
    pub id: i32,
    pub name: String,
    pub persona: String,
    pub example_dialogue: String,
    pub first_message: String,
    pub long_term_mem: usize,
    pub short_term_mem: usize,
    pub roleplay: bool,
    pub dialogue_tuning: bool,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompanionView {
    pub name: String,
    pub persona: String,
    pub example_dialogue: String,
    pub first_message: String,
    pub long_term_mem: usize,
    pub short_term_mem: usize,
    pub roleplay: bool,
    pub dialogue_tuning: bool,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub persona: String,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserView {
    pub name: String,
    pub persona: String,
    pub avatar_path: String,
}

pub struct Database {}

impl Database {
    pub fn new() -> Result<usize> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ai BOOLEAN,
                content TEXT,
                created_at DATETIME
            )", []
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS companion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                persona TEXT,
                example_dialogue TEXT,
                first_message TEXT,
                long_term_mem INTEGER,
                short_term_mem INTEGER,
                roleplay BOOLEAN,
                dialogue_tuning BOOLEAN,
                avatar_path TEXT
            )", []
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                persona TEXT,
                avatar_path TEXT
            )", []
        )?;
        if Database::is_table_empty("companion", &con)? {
            con.execute(
                "INSERT INTO companion (name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, dialogue_tuning, avatar_path) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                &[
                    "Companion",
                    "I am an AI companion.",
                    "Hello, my name is Companion. I am here to help you with your AI needs. How can I help you today?",
                    "Hello, my name is Companion. I am here to help you with your AI needs. How can I help you today?",
                    5,
                    10,
                    true,
                    true,
                    "/assets/companion_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("user", &con)? {
            con.execute(
                "INSERT INTO user (name, persona, avatar_path) VALUES (?, ?, ?)",
                &[
                    "User",
                    "I am an AI user.",
                    "/assets/user_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("messages", &con)? {
            let first_message: String = con.prepare("SELECT first_message FROM companion")?.query_row([], |row| row.get(0))?;
            con.execute(
                "INSERT INTO messages (ai, content, created_at) VALUES (?, ?, ?)",
                &[
                    true,
                    first_message,
                    Local::now()
                ]
            )
        }
    }

    pub fn is_table_empty(table_name: &str, con: &Connection) -> Result<bool> {
        let mut stmt = con.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
        let mut rows = stmt.query([])?;
        let count: i64 = rows.next()?.unwrap().get(0)?;
        Ok(count == 0)
    }

    pub fn get_messages(&self) -> Result<Vec<Message>> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages")?;
        let rows = stmt.query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    }

    pub fn get_x_messages(x: usize, index: usize) -> Result<Vec<Message>> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages ORDER BY id DESC LIMIT ? OFFSET ?")?;
        let rows = stmt.query_map([x], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    }

    pub fn get_companion_data(&self) -> Result<CompanionView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, dialogue_tuning, avatar_path FROM companion LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(CompanionView {
                name: row.get(0)?,
                persona: row.get(1)?,
                example_dialogue: row.get(2)?,
                first_message: row.get(3)?,
                long_term_mem: row.get(4)?,
                short_term_mem: row.get(5)?,
                roleplay: row.get(6)?,
                dialogue_tuning: row.get(7)?,
                avatar_path: row.get(8)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_user_data(&self) -> Result<UserView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona, avatar_path FROM user LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(User {
                name: row.get(0)?,
                persona: row.get(1)?,
                avatar_path: row.get(2)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_message(&self, id: i32) -> Result<Message> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages WHERE id = ?")?;
        let row = stmt.query_row([id], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    pub fn insert_message(&self, message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "INSERT INTO messages (ai, content, created_at) VALUES (?, ?, ?)",
            &[
                message.ai,
                message.content,
                Local::now()
            ]
        )?;
        Ok(())
    }

    pub fn edit_message(&self, id: i32, message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE messages SET ai = ?, content = ? WHERE id = ?",
            &[
                message.ai,
                message.content,
                id
            ]
        )?;
        Ok(())
    }

    pub fn delete_message(&self, id: i32) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages WHERE id = ?",
            &[id],
        )?;
        Ok(())
    }

    pub fn delete_latest_message(&self) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages ORDER BY id DESC LIMIT 1",
            []
        )?;
        Ok(())
    }

    pub fn erase_messages(&self) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages",
            []
        )?;
        Ok(())
    }

    pub fn edit_companion(&self, companion: CompanionView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET name = ?, persona = ?, example_dialogue = ?, first_message = ?, long_term_mem = ?, short_term_mem = ?, roleplay = ?, dialogue_tuning = ?, avatar_path = ? WHERE id = ?",
            &[
                companion.name,
                companion.persona,
                companion.example_dialogue,
                companion.first_message,
                companion.long_term_mem,
                companion.short_term_mem,
                companion.roleplay,
                companion.dialogue_tuning,
                companion.avatar_path,
                0
            ]
        )?;
        Ok(())
    }

    pub fn edit_user(&self, user: UserView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE user SET name = ?, persona = ?, avatar_path = ? WHERE id = ?",
            &[
                user.name,
                user.persona,
                user.avatar_path,
                0
            ]
        )?;
        Ok(())
    }
}
