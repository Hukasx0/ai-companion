use actix_web::{get, post, delete, put, App, web, HttpResponse, HttpServer};


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

    HttpResponse::Ok().body(format!("{} {}", start_index, limit))
}

#[delete("/api/message")]
async fn clear_messages() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/api/message/{id}")]
async fn message_id(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}!", id))
}

#[put("/api/message/{id}")]
async fn message_put(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}!", id))
}

#[delete("/api/message/{id}")]
async fn message_delete(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}!", id))
}

#[post("/api/message/{id}")]
async fn message_post(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}!", id))
}


//              Companion

#[get("/api/companion")]
async fn companion() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[put("/api/companion")]
async fn companion_put() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/api/companion/card")]
async fn companion_card() -> HttpResponse {
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
async fn companion_avatar() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}


//              User

#[get("/api/user")]
async fn user() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
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
    let hostname: &str = "127.0.0.1";

    println!("Started Rust backend server at:\n -> http://{}:{}/", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(js)
            .service(css)
            .service(project_logo)
    })
    .bind((hostname, port))?
    .run()
    .await
}
