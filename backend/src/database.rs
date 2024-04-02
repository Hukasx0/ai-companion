use rusqlite::{Connection, Error, Result};
use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use crate::character_card::CharacterCard;


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub ai: bool,
    pub content: String,
    pub created_at: String,
}

fn get_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%A %d.%m.%Y %H:%M").to_string()
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
}

#[derive(Serialize, Deserialize)]
pub struct UserView {
    pub name: String,
    pub persona: String,
}

#[derive(PartialEq)]
pub enum Device {
    CPU,
    GPU,
    Metal,
}

impl FromSql for Device {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Integer(i) => match i {
                0 => Ok(Device::CPU),
                1 => Ok(Device::GPU),
                2 => Ok(Device::Metal),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(PartialEq)]
pub enum PromptTemplate {
    Default,
    Llama2,
    Mistral
}

impl FromSql for PromptTemplate {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            rusqlite::types::ValueRef::Integer(i) => match i {
                0 => Ok(PromptTemplate::Default),
                1 => Ok(PromptTemplate::Llama2),
                2 => Ok(PromptTemplate::Mistral),
                _ => Err(FromSqlError::OutOfRange(i)),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

struct Config {
    id: i32,
    device: Device,
    llm_model_path: String,
    prompt_template: PromptTemplate
}

pub struct ConfigView {
    pub device: Device,
    pub llm_model_path: String,
    pub prompt_template: PromptTemplate
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
                created_at TEXT
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
        con.execute(
            "CREATE TABLE IF NOT EXISTS config (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device INTEGER,
                llm_model_path TEXT,
                prompt_template INTEGER
            )", []
        )?;
        if Database::is_table_empty("companion", &con)? {
            con.execute(
                "INSERT INTO companion (name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, dialogue_tuning, avatar_path) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                &[
                    "Assistant",
                    "{{char}} is an artificial intelligence chatbot designed to help {{user}}. {{char}} is an artificial intelligence created in ai-companion backend",
                    "{{user}}: What is ai-companion?\n{{char}}: AI Companion is a open-source project, wrote in Rust, Typescript and React, that aims to provide users with their own personal AI chatbot on their computer. It allows users to engage in friendly and natural conversations with their AI, creating a unique and personalized experience. This software can also be used as a backend or API for other projects that require a personalised AI chatbot. Very light size, simple installation, simple configuration, quick cold start and ease of use are some of the strengths of AI Companion in comparison to other similar projects.\n{{user}}: Can you tell me about the creator of ai-companion?\n{{char}}: the creator of the ai-companion program is 'Hubert Kasperek', he is a young programmer from Poland who is mostly interested in web development and computer science concepts, he has account on GitHub under nickname \"Hukasx0\"",
                    "Hello {{user}}, how can i help you today?",
                    "2",
                    "5",
                    "true",
                    "true",
                    "/assets/companion_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("user", &con)? {
            con.execute(
                "INSERT INTO user (name, persona, avatar_path) VALUES (?, ?, ?)",
                &[
                    "User",
                    "{{user}} is chatting with {{char}} using ai-companion web user interface",
                    "/assets/user_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("messages", &con)? {
            let first_message: String = con.prepare("SELECT first_message FROM companion")?.query_row([], |row| row.get(0))?;
            con.execute(
                "INSERT INTO messages (ai, content, created_at) VALUES (?, ?, ?)",
                &[
                    &true.to_string(),
                    &first_message,
                    &get_current_date()
                ]
            )?;
        }
        if Database::is_table_empty("config", &con)? {
            con.execute(
                "INSERT INTO config (device, llm_model_path, prompt_template) VALUES (?, ?, ?)",
                &[
                    &(Device::CPU as i32).to_string(),
                    "models/llama2-7b.gguf",
                    &(PromptTemplate::Default as i32).to_string()
                ]
            )?;
        } 
        Ok(0)
    }

    pub fn is_table_empty(table_name: &str, con: &Connection) -> Result<bool> {
        let mut stmt = con.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
        let mut rows = stmt.query([])?;
        let count: i64 = rows.next()?.unwrap().get(0)?;
        Ok(count == 0)
    }

    pub fn get_messages() -> Result<Vec<Message>> {
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
        let rows = stmt.query_map([x, index], |row| {
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

    pub fn get_companion_data() -> Result<CompanionView> {
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

    pub fn get_companion_card_data() -> Result<CharacterCard> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona, example_dialogue, first_message FROM companion LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(CharacterCard {
                name: row.get(0)?,
                description: row.get(1)?,
                first_mes: row.get(2)?,
                mes_example: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_user_data() -> Result<UserView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona FROM user LIMIT 1")?;
        let row: UserView = stmt.query_row([], |row| {
            Ok(UserView {
                name: row.get(0)?,
                persona: row.get(1)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_message(id: i32) -> Result<Message> {
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

    pub fn insert_message(message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "INSERT INTO messages (ai, content, created_at) VALUES (?, ?, ?)",
            &[
                &message.ai.to_string(),
                &message.content,
                &get_current_date()
            ]
        )?;
        Ok(())
    }

    pub fn edit_message(id: i32, message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE messages SET ai = ?, content = ? WHERE id = ?",
            &[
                &message.ai.to_string(),
                &message.content,
                &id.to_string()
            ]
        )?;
        Ok(())
    }

    pub fn delete_message(id: i32) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages WHERE id = ?",
            [id],
        )?;
        Ok(())
    }

    pub fn delete_latest_message() -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages ORDER BY id DESC LIMIT 1",
            []
        )?;
        Ok(())
    }

    pub fn erase_messages() -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages",
            []
        )?;
        Ok(())
    }

    pub fn edit_companion(companion: CompanionView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET name = ?, persona = ?, example_dialogue = ?, first_message = ?, long_term_mem = ?, short_term_mem = ?, roleplay = ?, dialogue_tuning = ?, avatar_path = ? WHERE id = ?",
            &[
                &companion.name,
                &companion.persona,
                &companion.example_dialogue,
                &companion.first_message,
                &companion.long_term_mem.to_string(),
                &companion.short_term_mem.to_string(),
                &companion.roleplay.to_string(),
                &companion.dialogue_tuning.to_string(),
                &companion.avatar_path,
                "0"
            ]
        )?;
        Ok(())
    }

    pub fn import_character_json(companion: CharacterCard) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "INSERT INTO companion (name, persona, example_dialogue, first_message) VALUES (?, ?, ?, ?)",
            &[
                &companion.name,
                &companion.description,
                &companion.first_mes,
                &companion.mes_example
            ]
        )?;
        Ok(())
    }

    pub fn import_character_card(companion: CharacterCard, image_path: &str) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "INSERT INTO companion (name, persona, example_dialogue, first_message, avatar_path) VALUES (?, ?, ?, ?, ?)",
            &[
                &companion.name,
                &companion.description,
                &companion.first_mes,
                &companion.mes_example,
                image_path
            ]
        )?;
        Ok(())
    }
        

    pub fn change_companion_avatar(avatar_path: &str) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET avatar_path = ? WHERE id = ?",
            &[
                avatar_path,
                "0"
            ]
        )?;
        Ok(())
    }

    pub fn edit_user(user: UserView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE user SET name = ?, persona = ? WHERE id = ?",
            &[
                &user.name,
                &user.persona,
                "0"
            ]
        )?;
        Ok(())
    }

    pub fn get_config() -> Result<ConfigView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT device, llm_model_path, prompt_template FROM config LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(ConfigView {
                device: row.get(0)?,
                llm_model_path: row.get(1)?,
                prompt_template: row.get(2)?
            })
        })?;
        Ok(row)
    }

    pub fn change_config(config: ConfigView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE config SET device = ?, llm_model_path = ? WHERE id = ?",
            &[
                &(config.device as i32).to_string(),
                &config.llm_model_path,
                "0"
            ]
        )?;
        Ok(())
    }
}
