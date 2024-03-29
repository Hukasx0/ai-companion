use actix_web::{get, post, delete, put, App, web, HttpResponse, HttpServer};
use crate::database::Database;
use crate::memory::LongTermMem;


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
    match Database::clear_messages() {
        Ok(_) => HttpResponse::Ok().body("Chat log cleared!"),
        Err(e) => {
            println!("Failed to clear chat log: {}", e);
            HttpResponse::InternalServerError().body("Error while clearing chat log, check logs for more information")
        }
    }
}

#[get("/api/message/{id}")]
async fn message_id(id: web::Path<String>) -> HttpResponse {
    let message: Message = match Database::get_message(id) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get message at id {}: {}", id, e);
            return HttpResponse::InternalServerError().body(format!("Error while getting message at id {}, check logs for more information", id));
        }
    };
    let message_json = serde_json::to_string(&message).unwrap_or(String::from("Error serializing message as JSON"));
    HttpResponse::Ok().body(message_json)
}

#[put("/api/message/{id}")]
async fn message_put(id: web::Path<String>, received: web::Json<NewMessage>) -> HttpResponse {
    match Database::edit_message(id, received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Message edited at id {}!", id)),
        Err(e) => {
            println!("Failed to edit message at id {}: {}", id, e);
            HttpResponse::InternalServerError().body(format!("Error while editing message at id {}, check logs for more information", id))
        }
    }
}

#[delete("/api/message/{id}")]
async fn message_delete(id: web::Path<String>) -> HttpResponse {
    match Database::delete_message(id) {
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
    match Database::edit_companion_data(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Companion data edited!"),
        Err(e) => {
            println!("Failed to edit companion data: {}", e);
            HttpResponse::InternalServerError().body("Error while editing companion data, check logs for more information")
        }
    }
}

#[derive(Deserialize)]
struct CharacterCard {
    name: String,
    description: String,
    first_mes: String,
    mes_example: String,
}

#[post("/api/companion/card")]
async fn companion_card(mut received: actix_web::web::Payload) -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/api/companion/card")]
async fn get_companion_card() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/companion/characterJson")]
async fn companion_character_json() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/api/companion/characterJson")]
async fn get_companion_character_json() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/companion/avatar")]
async fn companion_avatar(mut received: actix_web::web::Payload) -> HttpResponse {
    // curl -X POST -H "Content-Type: image/png" -T card.png http://localhost:3000/api/change/companionAvatar
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
    let user_data: User = match Database::get_user_data() {
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
async fn user_put() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}



//              Memory

#[put("/api/memory/shortTerm")]
async fn memory_long_term() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/memory/longTerm")]
async fn add_memory_long_term_message() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[delete("/api/memory/longTerm")]
async fn erase_long_term() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[put("/api/memory/dialogueTuning")]
async fn dialogue_tuning() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/memory/dialogueTuning")]
async fn add_tuning_message() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[delete("/api/memory/dialogueTuning")]
async fn erase_tuning_message() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}


//              Prompting

#[get("/api/prompt")]
async fn prompt() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/prompt")]
async fn prompt_message() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/api/prompt/regenerate")]
async fn regenerate_prompt() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
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

    // try to load character from "character.card.png"
    // if not then try to load character from "character.json"
    // if not, then skip, and load default character

    println!("\nAI Companion v1 successfully launched! 🚀\n");

    println!("Started Rust backend server at:\n -> http://{}:{}/", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(js)
            .service(css)
            .service(project_logo)
            .service(message)
            .service(clear_messages)
            .service(message_id)
            .service(message_put)
            .service(message_delete)
            .service(message_post)
            .service(companion)
            .service(companion_edit_data)
            .service(companion_card)
            .service(get_companion_card)
            .service(companion_character_json)
            .service(get_companion_character_json)
            .service(companion_avatar)
            .service(user)
            .service(user_put)
            .service(memory_long_term)
            .service(add_memory_long_term_message)
            .service(erase_long_term)
            .service(dialogue_tuning)
            .service(add_tuning_message)
            .service(erase_tuning_message)
            .service(prompt)
            .service(prompt_message)
            .service(regenerate_prompt)
    })
    .bind((hostname, port))?
    .run()
    .await
}
