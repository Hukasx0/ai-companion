use rusqlite::{Connection, Result, Error};
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
    pub example_dialogue: String,
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
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                persona TEXT NOT NULL
            )", [],
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS companion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                persona TEXT NOT NULL,
                example_dialogue TEXT NOT NULL,
                first_message TEXT NOT NULL,
                long_term_mem INTEGER NOT NULL,
                short_term_mem INTEGER NOT NULL,
                roleplay INTEGER NOT NULL,
                avatar_path STRING NOT NULL
            )", [],
        )?;
        if Database::is_table_empty("companion", &con) {
            con.execute(
                "INSERT INTO companion (id, name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, avatar_path) VALUES (NULL, \"Assistant\", \"Assistant is an artificial intelligence model designed to help the user\", \"\", \"hello user, how can i help you?\", 2, 5, 1, \"/assets/companion_avatar-4rust.jpg\")", []
            )?;
        }
        if Database::is_table_empty("user", &con) {
            con.execute(
                "INSERT INTO user (id, name, persona) VALUES (NULL, \"user\", \"user chatting with artificial intelligence\")", []
            )?;
        }
        if Database::is_table_empty("messages", &con) {
            let local: DateTime<Local> = Local::now();
            let formatted_date = local.format("%A %d.%m.%Y %H:%M").to_string();
            let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0))?;
            con.execute(
                &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", ?1, \"{}\")", formatted_date), [&first_message]
            )
        } else {
            Ok(0)
        }
    }

    pub fn is_table_empty(table_name: &str, con: &Connection) -> bool {
        let count: i64 = con.query_row(&format!("SELECT COUNT(*) FROM {}",table_name), [], |row| row.get(0)).unwrap();
        count == 0
    }

    pub fn get_messages() -> Result<Vec<Message>> {
        let con = Connection::open("companion.db")?;
        let mut stmt = con.prepare("SELECT id, ai, text, date FROM messages")?;
        let message_rows = stmt.query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                text: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        let mut messages: Vec<Message> = Vec::new();
        for msgs in message_rows {
           messages.push(msgs?);
        }
        Ok(messages)
    }

    pub fn get_x_msgs(msgs_limit: u32) -> Result<Vec<Message>> {
        let con = Connection::open("companion.db")?;
        let mut stmt = con.prepare(&format!("SELECT id, ai, text, date FROM messages ORDER BY id DESC LIMIT {}", msgs_limit))?;
        let message_rows = stmt
        .query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                text: row.get(2)?,
                date: row.get(3)?,
            })
        })?;
        let mut messages: Vec<Message> = Vec::new();
        for msgs in message_rows {
           messages.push(msgs?);
        }
        Ok(messages.into_iter().rev().collect())
    }

    pub fn get_companion_data() -> Result<CompanionData> {
        let con = Connection::open("companion.db")?;
        let mut stmt = con.prepare("SELECT * FROM companion LIMIT 1")?;
        let companion_data = stmt
        .query_map([], |row| {
            Ok(CompanionData {
                id: row.get(0)?,
                name: row.get(1)?,
                persona: row.get(2)?,
                example_dialogue: row.get(3)?,
                first_message: row.get(4)?,
                long_term_mem: row.get(5)?,
                short_term_mem: row.get(6)?,
                roleplay: row.get(7)?,
                avatar_path: row.get(8)?,
            })
        })?;
        let mut result: CompanionData = Default::default();
        for companion in companion_data {
            result = companion?;
         }
        Ok(result)
    }

    pub fn get_user_data() -> Result<UserData> {
        let con = Connection::open("companion.db")?;
        let mut stmt = con.prepare("SELECT * FROM user LIMIT 1")?;
        let user_data = stmt
        .query_map([], |row| {
            Ok(UserData {
                id: row.get(0)?,
                name: row.get(1)?,
                persona: row.get(2)?,
            })
        })?;
        let mut result: UserData = Default::default();
        for user in user_data {
            result = user?;
         }
        Ok(result)
    }

    pub fn add_message(text: &str, is_ai: bool) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        let ai = &is_ai.to_string();
        let local: DateTime<Local> = Local::now();
        let formatted_date = &local.format("%A %d.%m.%Y %H:%M").to_string();
        con.execute("INSERT INTO messages (id, ai, text, date) VALUES (NULL, ?1, ?2, ?3)", [&ai.as_str(), &text, &formatted_date.as_str()])?;
        Ok(())
    }

    pub fn clear_messages() -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute("DELETE FROM messages", [])?;
        let first_message: String = con.query_row("SELECT first_message FROM companion ASC LIMIT 1", [], |row| row.get(0))?;
        let local: DateTime<Local> = Local::now();
        let formatted_date = &local.format("%A %d.%m.%Y %H:%M").to_string();
        let companion = Database::get_companion_data()?;
        let user = Database::get_user_data()?;
        con.execute(
            &format!("INSERT INTO messages (id, ai, text, date) VALUES (NULL, \"true\", ?1, \"{}\")", formatted_date), [&first_message.replace("{{char}}", &companion.name).replace("{{user}}", &user.name)]
        )?;
        Ok(())
    }

    pub fn change_first_message(first_message: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET first_message=\"{}\"", first_message), [])?;
        Ok(())
    }

    pub fn change_companion_name(name: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET name=\"{}\"", name), [])?;
        Ok(())
    }

    pub fn change_companion_persona(persona: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET persona=\"{}\"", persona), [])?;
        Ok(())
    }

    pub fn change_companion_example_dialogue(example_dialogue: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET example_dialogue=\"{}\"", example_dialogue), [])?;
        Ok(())
    }

    pub fn change_companion(name: &str, persona: &str, example_dialogue: &str, first_message: &str, long_term_mem: u32, short_term_mem: u32, roleplay: bool) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET name=?1, persona=?2, example_dialogue=?3, first_message=?4, long_term_mem={}, short_term_mem={}, roleplay={}", long_term_mem, short_term_mem, roleplay), [&name, &persona, &example_dialogue, &first_message])?;
        Ok(())
    }

    pub fn change_companion_avatar(path: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET avatar_path=\"{}\"", path), [])?;
        Ok(())
    }

    pub fn import_companion(name: &str, persona: &str, example_dialogue: &str, first_message: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute("UPDATE companion SET name=?1, persona=?2, example_dialogue=?3, first_message=?4", [&name, &persona, &example_dialogue, &first_message])?;
        Ok(())
    }

    pub fn rm_message(id: u32) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("DELETE FROM messages WHERE id={}", id), [])?;
        Ok(())
    }

    pub fn change_username(name: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE user SET name=\"{}\"", name), [])?;
        Ok(())
    }

    pub fn change_user_persona(persona: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE user SET persona=\"{}\"", persona), [])?;
        Ok(())
    }

    pub fn change_user(name: &str, persona: &str) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE user SET name=\"{}\", persona=\"{}\"", name, persona), [])?;
        Ok(())
    }

    pub fn change_short_term_memory(limit: u32) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET short_term_mem={}", limit), [])?;
        Ok(())
    }

    pub fn change_long_term_memory(limit: u32) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET long_term_mem={}", limit), [])?;
        Ok(())
    }

    pub fn disable_enable_roleplay(op: bool) -> Result<(), Error> {
        let con = Connection::open("companion.db")?;
        con.execute(&format!("UPDATE companion SET roleplay={}", op), [])?;
        Ok(())
    }
}
