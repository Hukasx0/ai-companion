use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use llm::{Model, InferenceSession};
use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use std::fs;
mod database;
use database::{Database, Message, CompanionData, UserData};
mod vectordb;
use vectordb::VectorDatabase;

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

#[derive(Serialize)]
struct PromptResponse {
    id: u32,
    ai: bool,
    text: String,
    date: String,
}

#[post("/api/prompt")]
async fn test_prompt(received: web::Json<ReceivedPrompt>) -> HttpResponse {

    Database::add_message(&received.prompt, false);
    let vector = VectorDatabase::connect().unwrap();
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("* at %A %d.%m.%Y %H:%M*\n").to_string();
    let mut is_llama2: bool = false;

    // https://github.com/rustformers/llm
    // https://docs.rs/llm/latest/llm/

    // get .bin file (ai model) from models/ folder
    let mut model_path: String = String::from("");
    let dir_path = "models/";
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".bin") {
                        model_path = "models/".to_owned()+file_name;
                        println!("loaded model models/{}", file_name);
                        is_llama2 = file_name.contains("llama");
                    }
                }
            }
        }
    }
    if model_path == "" {
        eprintln!("You need to put your AI model (with .bin format - ggml) in models/ folder");
        panic!();
    }

    let llama = llm::load::<llm::models::Llama>(
        std::path::Path::new(&model_path),
        llm::TokenizerSource::Embedded,
        llm::ModelParameters::default(),
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    
    let mut session = llama.start_session(Default::default());
    let mut x = String::new();
    println!("Generating ai response...");
    let companion: CompanionData = Database::get_companion_data();
    let user: UserData = Database::get_user_data();
    let mut base_prompt: String = String::new();
    let mut rp: &str = "";
    if companion.roleplay == 1 {
        rp = "gestures and other non-verbal actions are written between asterisks (for example, *waves hello* or *moves closer*)";
    }
    if is_llama2 {
        base_prompt = 
        format!("<<SYS>>\nYou are {}, {}\nyou are talking with {}, {} is {}\n{}",
                companion.name, companion.persona, user.name, user.name, user.persona, rp);
    } else {
        base_prompt = 
        format!("Text transcript of a conversation between {} and {}. {}\n{}'s Persona: {}\n{}'s Persona: {}\n<START>\n", 
                                            user.name, companion.name, rp, user.name, user.persona, companion.name, companion.persona);
    }
    let abstract_memory: Vec<String> = vector.get_matches(&received.prompt, companion.long_term_mem);
    for message in abstract_memory {
        base_prompt += &message;
    }
    let ai_memory: Vec<Message> = Database::get_x_msgs(companion.short_term_mem);
    if is_llama2 {
        for message in ai_memory {
            let prefix = if message.ai == "true" { &companion.name } else { &user.name };
            let text = message.text;
            let formatted_message = format!("{}: {}\n", prefix, text);
            base_prompt += &("[INST]".to_owned() + &formatted_message + "[/INST]\n");
        }
        base_prompt += "<</SYS>>";
    } else {
        for message in ai_memory {
            let prefix = if message.ai == "true" { &companion.name } else { &user.name };
            let text = message.text;
            let formatted_message = format!("{}: {}\n", prefix, text);
            base_prompt += &formatted_message;
        }
    }
    let res = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: llm::Prompt::Text(&format!("{}{}:", &base_prompt, companion.name)),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        |t| {
            match t {
                llm::InferenceResponse::SnapshotToken(token) => {print!("{token}");}
                llm::InferenceResponse::PromptToken(token) => {print!("{token}");}
                llm::InferenceResponse::InferredToken(token) => {
                    x = (x.clone()+&token).to_string();
                    print!("{token}");
                }
                llm::InferenceResponse::EotToken => {}
            }
            std::io::stdout().flush().unwrap();
            Ok(llm::InferenceFeedback::Continue)
        }
    );
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    let companion_text = x
    .split(&format!("\n{}: ", &companion.name))
    .next()
    .unwrap_or("");
    Database::add_message(&companion_text, true);
    vector.add_entry(&format!("{}{}: {}\n{}: {}\n", formatted_date,user.name, &received.prompt, &companion.name, &companion_text));
    return HttpResponse::Ok().body(serde_json::to_string(&PromptResponse {
        id: 0,
        ai: true,
        text: companion_text.to_string(),
        date: String::from("now"),
    }).unwrap())
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

#[derive(Deserialize)]
struct RmMsg {
    id: u32,
}

#[post("/api/removeMessage")]
async fn rm_message(received: web::Json<RmMsg>) -> HttpResponse {
    Database::rm_message(received.id);
    HttpResponse::Ok().body("Removed message")
}

#[get("/api/companionData")]
async fn fetch_companion_data() -> HttpResponse {
    HttpResponse::Ok().body(serde_json::to_string(&Database::get_companion_data()).unwrap())
}

#[get("/api/userData")]
async fn fetch_user_data() -> HttpResponse {
    HttpResponse::Ok().body(serde_json::to_string(&Database::get_user_data()).unwrap())
}

#[derive(Deserialize)]
struct ChangeFirstMessage {
    first_message: String,
}

#[post("/api/change/firstMessage")]
async fn change_first_message(received: web::Json<ChangeFirstMessage>) -> HttpResponse {
    Database::change_first_message(&received.first_message);
    HttpResponse::Ok().body("Changed first message")
}

#[derive(Deserialize)]
struct ChangeCompanionName {
    companion_name: String,
}

#[post("/api/change/companionName")]
async fn change_companion_name(received: web::Json<ChangeCompanionName>) -> HttpResponse {
    Database::change_companion_name(&received.companion_name);
    HttpResponse::Ok().body("Changed companion name")
}

#[derive(Deserialize)]
struct ChangeUsername {
    user_name: String,
}

#[post("/api/change/userName")]
async fn change_user_name(received: web::Json<ChangeUsername>) -> HttpResponse {
    Database::change_username(&received.user_name);
    HttpResponse::Ok().body("Changed user name")
}

#[derive(Deserialize)]
struct ChangeCompanionPersona {
    companion_persona: String,
}

#[post("/api/change/companionPersona")]
async fn change_companion_persona(received: web::Json<ChangeCompanionPersona>) -> HttpResponse {
    Database::change_companion_persona(&received.companion_persona);
    HttpResponse::Ok().body("Changed companion persona")
}

#[derive(Deserialize)]
struct ChangeUserPersona {
    user_persona: String,
}

#[post("/api/change/userPersona")]
async fn change_user_persona(received: web::Json<ChangeUserPersona>) -> HttpResponse {
    Database::change_user_persona(&received.user_persona);
    HttpResponse::Ok().body("Changed user persona")
}

#[derive(Deserialize, Debug)]
struct ChangeCompanionData {
    id: u32,
    name: String,
    persona: String,
    first_message: String,
    long_term_mem: u32,
    short_term_mem: u32,
    roleplay: bool,
}

#[post("/api/change/companionData")]
async fn change_companion_data(received: web::Json<ChangeCompanionData>) -> HttpResponse {
    Database::change_companion(&received.name, &received.persona, &received.first_message, received.long_term_mem, received.short_term_mem, received.roleplay);
    HttpResponse::Ok().body("Data of your ai companion has been changed")
}

#[derive(Deserialize)]
struct ChangeUserData {
    id: u32,
    name: String,
    persona: String,
}

#[post("/api/change/userData")]
async fn change_user_data(received: web::Json<ChangeUserData>) -> HttpResponse {
    Database::change_user(&received.name, &received.persona);
    HttpResponse::Ok().body("Data of user has been changed")
}

#[derive(Deserialize)]
struct AddData {
    text: String,
}

#[post("/api/addData")]
async fn add_custom_data(received: web::Json<AddData>) -> HttpResponse {
    match VectorDatabase::connect() {
        Ok(vdb) => {
            vdb.add_entry(&(received.text.to_string()+"\n"));
            HttpResponse::Ok().body("Added custom data to AI long term memory")
        }
        Err(_) => HttpResponse::Ok().body("Error while connecting with AI long term memory")
    }
}

#[get("/api/erase/longTermMemory")]
async fn erase_longterm_mem() -> HttpResponse {
    match VectorDatabase::connect() {
        Ok(vdb) => {
            vdb.erase_memory();
            HttpResponse::Ok().body("Erased AI's long term memory")
        }
        Err(_) => HttpResponse::Ok().body("Error while connecting with AI long term memory")
    }
}

#[derive(Deserialize)]
struct ChangeMemory {
    limit: u32,
}

#[post("/api/change/longTermMemory")]
async fn change_long_term_mem(received: web::Json<ChangeMemory>) -> HttpResponse {
    Database::change_long_term_memory(received.limit);
    HttpResponse::Ok().body("Changed long term memory limit for ai")
}

#[post("/api/change/shortTermMemory")]
async fn change_short_term_mem(received: web::Json<ChangeMemory>) -> HttpResponse {
    Database::change_short_term_memory(received.limit);
    HttpResponse::Ok().body("Changed short term memory limit for ai")
}

#[derive(Deserialize)]
struct ChangeRoleplay {
    enable: bool,
}

#[post("/api/change/roleplay")]
async fn change_roleplay(received: web::Json<ChangeRoleplay>) -> HttpResponse {
    Database::disable_enable_roleplay(received.enable);
    if received.enable {
        HttpResponse::Ok().body("Enabled roleplay")
    } else {
        HttpResponse::Ok().body("Disabled roleplay")
    }
}

// works with https://zoltanai.github.io/character-editor/
#[derive(Deserialize)]
struct CharacterJson {
    char_name: String,
    char_persona: String,
    char_greeting: String,
}

#[post("/api/import/characterJson")]
async fn import_character_json(received: web::Json<CharacterJson>) -> HttpResponse {
    Database::import_companion(&received.char_name, &received.char_persona, &received.char_greeting);
    HttpResponse::Ok().body("Data of your ai companion has been changed")
}

/*#[post("/api/import/characterCard")]
async fn change_roleplay(received: web::Json<ChangeRoleplay>) -> HttpResponse {
    Database::import_companion(&received.char_name, &received.char_persona, &received.char_greeting, received.long_term_mem, received.short_term_mem, received.roleplay);
    HttpResponse::Ok().body("Data of your ai companion has been changed")
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 3000;
    let hostname: &str = "0.0.0.0";

    match Database::create() {
        Ok(_) => { println!("Successfully connected to local database"); }
        Err(e) => { eprintln!("Cannot connect to SQLite database because of: {}",e); }
    }

    match VectorDatabase::connect() {
        Ok(_) => { println!("Successfully connected to tantivy"); }
        Err(e) => { eprintln!("Cannot connect to tantivy because of: {}",e); }
    }

    println!("AI companion works at:\n -> http://{}:{}/", hostname, port);
    println!("You can access it, by entering a link in your browser:\n -> http://localhost:{}/", port);
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
            .service(rm_message)
            .service(change_first_message)
            .service(change_companion_name)
            .service(change_companion_persona)
            .service(change_companion_data)
            .service(fetch_companion_data)
            .service(fetch_user_data)
            .service(change_user_data)
            .service(change_user_name)
            .service(change_user_persona)
            .service(add_custom_data)
            .service(erase_longterm_mem)
            .service(change_long_term_mem)
            .service(change_short_term_mem)
            .service(change_roleplay)
            .service(import_character_json)
    })
    .bind((hostname, port))?
    .run()
    .await
}
