use actix_web::{get, post, delete, put, App, web, HttpResponse, HttpServer};
use futures_util::StreamExt as _;
mod database;
use database::{Database, Message, NewMessage, CompanionView, UserView};
mod long_term_mem;
use long_term_mem::LongTermMem;
mod dialogue_tuning;
use dialogue_tuning::DialogueTuning;
mod character_card;
use character_card::CharacterCard;
use serde::Deserialize;
mod llm;
use crate::llm::prompt;

use std::fs;
use std::fs::File;
use std::io::Write;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../dist/index.html"))
}

#[get("/assets/index-4rust.js")]
async fn js() -> HttpResponse {
    HttpResponse::Ok().content_type("application/javascript").body(include_str!("../../dist/assets/index-4rust.js"))
}

#[get("/assets/index-4rust.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(include_str!("../../dist/assets/index-4rust.css"))
}

#[get("/ai_companion_logo.jpg")]
async fn project_logo() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/ai_companion_logo.jpg")[..])
}

#[get("/assets/companion_avatar-4rust.jpg")]
async fn companion_avatar_img() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/assets/companion_avatar-4rust.jpg")[..])
}


//              API


//              Message

#[derive(serde::Deserialize)]
struct MessageQuery {
    start_index: Option<usize>,
    limit: Option<usize>,
}

#[get("/api/message")]
async fn message(query_params: web::Query<MessageQuery>) -> HttpResponse {
    let start_index: usize = query_params.start_index.unwrap_or(0);

    // 50 Messages is the max
    let limit: usize = query_params.limit.unwrap_or(15).min(50);

    // query to database, and return messages
    let messages: Vec<Message> = match Database::get_x_messages(limit, start_index) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get messages from database: {}", e);
            return HttpResponse::InternalServerError().body("Error while getting messages from database, check logs for more information");
        },
    };
    let messages_json = serde_json::to_string(&messages).unwrap_or(String::from("Error serializing messages as JSON"));
    HttpResponse::Ok().body(messages_json)
}

#[post("/api/message")]
async fn message_post(received: web::Json<NewMessage>) -> HttpResponse {
    match Database::insert_message(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Message added!"),
        Err(e) => {
            println!("Failed to add message: {}", e);
            HttpResponse::InternalServerError().body("Error while adding message, check logs for more information")
        }
    }
}

#[delete("/api/message")]
async fn clear_messages() -> HttpResponse {
    match Database::erase_messages() {
        Ok(_) => HttpResponse::Ok().body("Chat log cleared!"),
        Err(e) => {
            println!("Failed to clear chat log: {}", e);
            HttpResponse::InternalServerError().body("Error while clearing chat log, check logs for more information")
        }
    }
}

#[get("/api/message/{id}")]
async fn message_id(id: web::Path<i32>) -> HttpResponse {
    let msg: Message = match Database::get_message(*id) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get message at id {}: {}", id, e);
            return HttpResponse::InternalServerError().body(format!("Error while getting message at id {}, check logs for more information", id));
        }
    };
    let message_json = serde_json::to_string(&msg).unwrap_or(String::from("Error serializing message as JSON"));
    HttpResponse::Ok().body(message_json)
}

#[put("/api/message/{id}")]
async fn message_put(id: web::Path<i32>, received: web::Json<NewMessage>) -> HttpResponse {
    match Database::edit_message(*id, received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Message edited at id {}!", id)),
        Err(e) => {
            println!("Failed to edit message at id {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error while editing message at id {}, check logs for more information", id))
        }
    }
}

#[delete("/api/message/{id}")]
async fn message_delete(id: web::Path<i32>) -> HttpResponse {
    match Database::delete_message(*id) {
        Ok(_) => HttpResponse::Ok().body(format!("Message deleted at id {}!", id)),
        Err(e) => {
            println!("Failed to delete message at id {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error while deleting message at id {}, check logs for more information", id))
        }
    }
}

//              Companion

#[get("/api/companion")]
async fn companion() -> HttpResponse {
    let companion_data: CompanionView = match Database::get_companion_data() {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get companion data: {}", e);
            return HttpResponse::InternalServerError().body("Error while getting companion data, check logs for more information");
        }
    };
    let companion_json: String = serde_json::to_string(&companion_data).unwrap_or(String::from("Error serializing companion data as JSON"));
    HttpResponse::Ok().body(companion_json)
}

#[put("/api/companion")]
async fn companion_edit_data(received: web::Json<CompanionView>) -> HttpResponse {
    match Database::edit_companion(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Companion data edited!"),
        Err(e) => {
            println!("Failed to edit companion data: {}", e);
            HttpResponse::InternalServerError().body("Error while editing companion data, check logs for more information")
        }
    }
}

#[post("/api/companion/card")]
async fn companion_card(mut received: actix_web::web::Payload) -> HttpResponse {
    // curl -X POST -H "Content-Type: image/png" -T card.png http://localhost:3000/api/companion/card
    let mut data = web::BytesMut::new();
    while let Some(chunk) = received.next().await {
        let d = chunk.unwrap();
        data.extend_from_slice(&d);
    }
    let character_card: CharacterCard = match CharacterCard::load_character_card(&data) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error while loading character card from a file: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    let mut avatar_file = match File::create("assets/avatar.png") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error while creating 'avatar.png' file in a 'assets' folder: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    match avatar_file.write_all(&data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while writing bytes to 'avatar.png' file in a 'assets' folder: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    match Database::change_companion_avatar("assets/avatar.png") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while changing companion avatar using character card: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    println!("Character \"{}\" imported successfully!", character_card.name);
    HttpResponse::Ok().body("Updated companion data via character card!")

}

#[post("/api/companion/characterJson")]
async fn companion_character_json(received: web::Json<CharacterCard>) -> HttpResponse {
    match Database::import_character_json(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Character json imported successfully!"),
        Err(e) => {
            println!("Failed to import character json: {}", e);
            HttpResponse::InternalServerError().body("Error while importing character json, check logs for more information")
        }
    }
}

#[get("/api/companion/characterJson")]
async fn get_companion_character_json() -> HttpResponse {
    match Database::get_companion_card_data() {
        Ok(v) => { 
            let character_json: String = serde_json::to_string(&v as &CharacterCard).unwrap_or(String::from("Error serializing companion data as JSON"));
            return HttpResponse::Ok().body(character_json);
        },
        Err(e) => {
            println!("Failed to get companion card data: {}", e);
            return HttpResponse::InternalServerError().body("Error while getting companion card data, check logs for more information");
        },
    };
}

#[post("/api/companion/avatar")]
async fn companion_avatar(mut received: actix_web::web::Payload) -> HttpResponse {
    // curl -X POST -H "Content-Type: image/png" -T avatar.png http://localhost:3000/api/companion/avatar
    let mut data = web::BytesMut::new();
    while let Some(chunk) = received.next().await {
        let d = chunk.unwrap();
        data.extend_from_slice(&d);
    }
    if fs::metadata("assets").is_err() {
        match fs::create_dir("assets") {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error while creating 'assets' directory: {}", e);
                return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
            }
        };
    }
    let mut avatar_file = match File::create("assets/avatar.png") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error while creating 'avatar.png' file in a 'assets' folder: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    match avatar_file.write_all(&data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while writing bytes to 'avatar.png' file in a 'assets' folder: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    match Database::change_companion_avatar("assets/avatar.png") {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while changing companion avatar: {}", e);
            return HttpResponse::InternalServerError().body("Error while changing companion avatar, check logs for more information");
        }
    };
    HttpResponse::Ok().body("Companion avatar changed!")
}


//              User

#[get("/api/user")]
async fn user() -> HttpResponse {
    let user_data: UserView = match Database::get_user_data() {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get user data: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let user_json: String = serde_json::to_string(&user_data).unwrap_or(String::from("Error serializing user data as JSON"));
    HttpResponse::Ok().body(user_json)
}

#[put("/api/user")]
async fn user_put(received: web::Json<UserView>) -> HttpResponse {
    match Database::edit_user(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("User data edited!"),
        Err(e) => {
            println!("Failed to edit user data: {}", e);
            HttpResponse::InternalServerError().body("Error while editing user data, check logs for more information")
        }
    }
}



//              Memory

#[derive(Deserialize)]
struct LongTermMemMessage {
    entry: String
}

#[post("/api/memory/longTerm")]
async fn add_memory_long_term_message(received: web::Json<LongTermMemMessage>) -> HttpResponse {
    let ltm = match LongTermMem::connect() {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to connect to long term memory: {}", e);
            return HttpResponse::InternalServerError().body("Error while connecting to long term memory, check logs for more information");
        }
    };
    match ltm.add_entry(&received.into_inner().entry) {
        Ok(_) => HttpResponse::Ok().body("Long term memory entry added!"),
        Err(e) => {
            println!("Failed to add long term memory entry: {}", e);
            HttpResponse::InternalServerError().body("Error while adding long term memory entry, check logs for more information")
        }
    }
}

#[delete("/api/memory/longTerm")]
async fn erase_long_term() -> HttpResponse {
    let ltm = match LongTermMem::connect() {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to connect to long term memory: {}", e);
            return HttpResponse::InternalServerError().body("Error while connecting to long term memory, check logs for more information");
        }
    };
    match ltm.erase_memory() {
        Ok(_) => HttpResponse::Ok().body("Long term memory cleared!"),
        Err(e) => {
            println!("Failed to clear long term memory: {}", e);
            HttpResponse::InternalServerError().body("Error while clearing long term memory, check logs for more information")
        }
    }
}

#[post("/api/memory/dialogueTuning")]
async fn add_tuning_message() -> HttpResponse {
    let messages = match Database::get_x_messages(2, 0) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get last 2 messages from database: {}", e);
            return HttpResponse::InternalServerError().body("Error while getting last 2 messages from database, check logs for more information");
        }
    };
    match DialogueTuning::insert(&messages[0].content, &messages[1].content) {
        Ok(_) => HttpResponse::Ok().body("Saved previous dialogue as template dialogue"),
        Err(e) => {
            println!("Failed to save previous dialogue as template dialogue: {}", e);
            HttpResponse::InternalServerError().body("Error while saving previous dialogue as template dialogue, check logs for more information")
        }
    }
}

#[delete("/api/memory/dialogueTuning")]
async fn erase_tuning_message() -> HttpResponse {
    match DialogueTuning::clear_dialogues() {
        Ok(_) => HttpResponse::Ok().body("Dialogue tuning memory cleared!"),
        Err(e) => {
            println!("Failed to clear dialogue tuning: {}", e);
            HttpResponse::InternalServerError().body("Error while clearing dialogue tuning, check logs for more information")
        }
    }
}


//              Prompting

#[derive(Deserialize)]
struct Prompt {
    prompt: String
}

#[post("/api/prompt")]
async fn prompt_message(received: web::Json<Prompt>) -> HttpResponse {
    match prompt(&received.into_inner().prompt) {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => {
            println!("Failed to generate prompt: {}", e);
            HttpResponse::InternalServerError().body("Error while generating prompt, check logs for more information")
        }
    }
}

#[get("/api/prompt/regenerate")]
async fn regenerate_prompt(received: web::Json<Prompt>) -> HttpResponse {
    match Database::delete_latest_message() {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to delete latest message: {}", e);
            return HttpResponse::InternalServerError().body("Error while deleting latest message, check logs for more information");
        }
    }
    match prompt(&received.into_inner().prompt) {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => {
            println!("Failed to re-generate prompt: {}", e);
            HttpResponse::InternalServerError().body("Error while generating prompt, check logs for more information")
        }
    }
}

//

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 3000;
    let hostname: &str = "0.0.0.0";

    match Database::new() {
        Ok(_) => { }
        Err(e) => eprintln!("⚠️ Failed to connect to sqlite database: {}", e),
    }

    match LongTermMem::connect() {
        Ok(_) => { }
        Err(e) => eprintln!("⚠️ Failed to connect to tantivy: {}", e),
    }

    match DialogueTuning::create() {
        Ok(_) => { }
        Err(e) => eprintln!("⚠️ Failed to create dialogue tuning table in sqlite database: {}", e),
    }

    println!("\nAI Companion v1 successfully launched! 🚀\n");

    println!("Started Rust backend server at:\n -> http://{}:{}/", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(js)
            .service(css)
            .service(project_logo)
            .service(companion_avatar_img)
            .service(message)
            .service(clear_messages)
            .service(message_id)
            .service(message_put)
            .service(message_delete)
            .service(message_post)
            .service(companion)
            .service(companion_edit_data)
            .service(companion_card)
            .service(companion_character_json)
            .service(get_companion_character_json)
            .service(companion_avatar)
            .service(user)
            .service(user_put)
            .service(add_memory_long_term_message)
            .service(erase_long_term)
            .service(add_tuning_message)
            .service(erase_tuning_message)
            .service(prompt_message)
            .service(regenerate_prompt)
    })
    .bind((hostname, port))?
    .run()
    .await
}
