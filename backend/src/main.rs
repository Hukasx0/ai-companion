use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use llm::Model;
use std::io::Write;
use serde::Deserialize;
mod database;
use database::{Database, Message};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../dist/index.html"))
}

#[get("/assets/companion_avatar-4rust.jpg")]
async fn companion_avatar() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/assets/companion_avatar-4rust.jpg")[..])
}

#[get("/ai_companion_logo.jpg")]
async fn project_logo() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/ai_companion_logo.jpg")[..])
}

#[get("/assets/index-4rust.js")]
async fn js() -> HttpResponse {
    HttpResponse::Ok().content_type("application/javascript").body(include_str!("../../dist/assets/index-4rust.js"))
}

#[get("/assets/index-4rust.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(include_str!("../../dist/assets/index-4rust.css"))
}

#[derive(Deserialize)]
struct ReceivedPrompt {
    prompt: String,
}

#[post("/api/testPrompt")]
async fn test_prompt(received: web::Json<ReceivedPrompt>) -> HttpResponse {

    Database::add_message(&received.prompt, false);

    // https://github.com/rustformers/llm
    // https://docs.rs/llm/latest/llm/

    let llama = llm::load::<llm::models::Llama>(
        // https://huggingface.co/TehVenom/Pygmalion-7b-4bit-Q4_1-GGML/tree/main
        std::path::Path::new("models/Pygmalion-7b-4bit-Q4_1-GGML-V2.bin"),
        Default::default(),
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    
    let mut session = llama.start_session(Default::default());
    let mut x = String::new();
    println!("Generating ai response...");
    let mut base_prompt: String = String::from("Assistant's Persona: Assistant is an artificial intelligence model designed to help the user\n<START>\n");
    let ai_memory: Vec<Message> = Database::get_five_msgs();
    for message in ai_memory {
        let prefix = if message.ai == "true" { "Assistant" } else { "user" };
        let text = message.text;
        let formatted_message = format!("{}: {}\n", prefix, text);
        base_prompt += &formatted_message;
    }
    let res = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: &format!("{}Assistant:", &base_prompt),
            ..Default::default()
        },
        &mut Default::default(),
        |t| {
            x = (x.clone()+t).to_string();
            print!("{t}");
            std::io::stdout().flush().unwrap();
            Ok(())
        }
    );
    
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    let companion_text = x
    .split("\nAssistant: ")
    .last()
    .unwrap_or("");
    Database::add_message(&companion_text, true);
    return HttpResponse::Ok().body(format!("{{\n\"id\": 0,\n\"ai\": true,\n\"text\": \"{}\",\n\"date\": \"now\"\n}}", companion_text.to_owned()));
}

#[get("/api/messages")]
async fn get_messages() -> HttpResponse {
    let messages: Vec<Message> = Database::get_messages();
    let json = serde_json::to_string(&messages).unwrap();
    HttpResponse::Ok().body(format!("{}", json))
}

#[get("/api/clearMessages")]
async fn clear_messages() -> HttpResponse {
    Database::clear_messages();
    HttpResponse::Ok().body("Chat log cleared")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 3000;
    let hostname: &str = "127.0.0.1";

    match Database::create() {
        Ok(_) => { println!("Successfully connected to local database"); }
        Err(e) => { eprintln!("Cannot connect ti SQLite database because of: {}",e); }
    }

    println!("Started Rust backend server at:\n -> http://{}:{}/", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(js)
            .service(css)
            .service(companion_avatar)
            .service(project_logo)
            .service(test_prompt)
            .service(get_messages)
            .service(clear_messages)
    })
    .bind((hostname, port))?
    .run()
    .await
}
