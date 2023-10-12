use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use llm::Model;
use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use futures_util::StreamExt as _;
use std::fs;
use std::fs::File;
use std::io::Read;
use base64::{Engine, engine::GeneralPurpose, engine::GeneralPurposeConfig, alphabet::STANDARD};
mod database;
use database::{Database, Message, CompanionData, UserData};
mod vectordb;
use vectordb::VectorDatabase;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../dist/index.html"))
}

#[get("/assets/companion_avatar-4rust.jpg")]
async fn companion_default_avatar() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/assets/companion_avatar-4rust.jpg")[..])
}

#[get("/assets/avatar.png")]
async fn companion_avatar() -> HttpResponse {
    let mut file = match File::open("assets/avatar.png") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error while opening 'assets/avatar.png' file: {}", e);
            return HttpResponse::InternalServerError().body("Error while sending image file, check logs for more information");
        }
    };
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while reading 'assets/avatar.png' file: {}", e);
            return HttpResponse::InternalServerError().body("Error while sending image file, check logs for more information");
        }
    };
    HttpResponse::Ok().content_type("image/png").body(buffer)
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

    match Database::add_message(&received.prompt, false) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while adding message to database/short-term memory: {}", e);
            return HttpResponse::InternalServerError().body("Error while generating output message, check logs for more information");
        },
    };
    let vector = match VectorDatabase::connect() {
        Ok(vd) => vd,
        Err(e) => {
            eprintln!("Error while connecting to tantivy: {}", e);
            return HttpResponse::InternalServerError().body("Error while generating output message, check logs for more information");
        }
    };
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("* at %A %d.%m.%Y %H:%M *\n").to_string();
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
    if model_path.is_empty() {
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
    let x: String;
    println!("Generating ai response...");
    let companion: CompanionData = match Database::get_companion_data() {
        Ok(cd) => cd,
        Err(e) => {
            eprintln!("Error while getting companion data from sqlite database: {}", e);
            return HttpResponse::InternalServerError().body("Error while generating output message, check logs for more information");
        }
    };
    let user: UserData = match Database::get_user_data() {
        Ok(ud) => ud,
        Err(e) => {
            eprintln!("Error while getting user data from sqlite database: {}", e);
            return HttpResponse::InternalServerError().body("Error while generating output message, check logs for more information");
        }
    };
    let mut base_prompt: String;
    let mut rp: &str = "";
    if companion.roleplay == 1 {
        rp = "gestures and other non-verbal actions are written between asterisks (for example, *waves hello* or *moves closer*)";
    }
    if is_llama2 {
        base_prompt = 
        format!("<<SYS>>\nYou are {}, {}\nyou are talking with {}, {} is {}\n{}\n[INST]\n{}\n[/INST]",
                companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), user.name, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), rp, companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name));
    } else {
        base_prompt = 
        format!("Text transcript of a conversation between {} and {}. {}\n{}'s Persona: {}\n{}'s Persona: {}\n<START>{}\n<START>\n", 
                                            user.name, companion.name, rp, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name));
    }
    let abstract_memory: Vec<String> = match vector.get_matches(&received.prompt, companion.long_term_mem) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error while getting messages from long-term memory: {}", e);
            Vec::new() // If there is a error with long-term memory, just display error, don't interrupt generation
        }
    };
    for message in abstract_memory {
        base_prompt += &message.replace("{{char}}", &companion.name).replace("{{user}}", &user.name);
    }
    let ai_memory: Vec<Message> = match Database::get_x_msgs(companion.short_term_mem) {
        Ok(msgs) => msgs,
        Err(e) => {
            eprintln!("Error while getting messages from database/short-term memory: {}", e);
            return HttpResponse::InternalServerError().body("Error while generating output message, check logs for more information");
        }
    };
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
    let mut end_of_generation = String::new();
    let eog = format!("\n{}:", user.name);
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
                llm::InferenceResponse::SnapshotToken(_) => {/*print!("{token}");*/}
                llm::InferenceResponse::PromptToken(_) => {/*print!("{token}");*/}
                llm::InferenceResponse::InferredToken(token) => {
                    //x = x.clone()+&token;
                    end_of_generation.push_str(&token);
                    print!("{token}");
                    if end_of_generation.contains(&eog) {
                        return Ok(llm::InferenceFeedback::Halt);          
                    }
                }
                llm::InferenceResponse::EotToken => {}
            }
            std::io::stdout().flush().unwrap();
            Ok(llm::InferenceFeedback::Continue)
        }
    );
    x = end_of_generation.replace(&eog, "");
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    let companion_text = x
    .split(&format!("\n{}: ", &companion.name))
    .next()
    .unwrap_or("");
    match Database::add_message(companion_text, true) {
        Ok(_) => {},
        Err(e) => eprintln!("Error while adding message to database/short-term memory: {}", e),
    };
    match vector.add_entry(&format!("{}{}: {}\n{}: {}\n", formatted_date, "{{user}}", &received.prompt, "{{char}}", &companion_text)) {
        Ok(_) => {},
        Err(e) => eprintln!("Error while adding message to long-term memory: {}", e),
    };
    HttpResponse::Ok().body(serde_json::to_string(&PromptResponse {
        id: 0,
        ai: true,
        text: companion_text.to_string(),
        date: String::from("now"),
    }).unwrap_or(String::from("Error while encoding companion response as json")))
}

#[get("/api/messages")]
async fn get_messages() -> HttpResponse {
    let messages: Vec<Message> = match Database::get_messages() {
        Ok(msgs) => msgs,
        Err(e) => {
            eprintln!("Error while getting messages from sqlite database: {}", e);
            Vec::new()
        },
    };
    let json = serde_json::to_string(&messages).unwrap_or(String::from("Error while encoding messages as json"));
    HttpResponse::Ok().body(json)
}

#[get("/api/clearMessages")]
async fn clear_messages() -> HttpResponse {
    match Database::clear_messages() {
        Ok(_) => {},
        Err(e) => eprintln!("Error while removing messages from sqlite database: {}", e),
    };
    HttpResponse::Ok().body("Chat log cleared")
}

#[derive(Deserialize)]
struct RmMsg {
    id: u32,
}

#[post("/api/removeMessage")]
async fn rm_message(received: web::Json<RmMsg>) -> HttpResponse {
    match Database::rm_message(received.id) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while removing message from sqlite database: {}", e);
            return HttpResponse::InternalServerError().body("Error while removing message, check logs for more information");
        },
    };
    HttpResponse::Ok().body("Removed message")
}

#[get("/api/companionData")]
async fn fetch_companion_data() -> HttpResponse {
    match Database::get_companion_data() {
        Ok(companion_data) => HttpResponse::Ok().body(serde_json::to_string(&companion_data).unwrap_or(String::from("Error while encoding companion data as json"))),
        Err(e) => {
            eprintln!("Error while getting companion data from sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while fetching companion data, check logs for more information")
        },
    }
}

#[get("/api/userData")]
async fn fetch_user_data() -> HttpResponse {
    match Database::get_user_data() {
        Ok(user_data) => HttpResponse::Ok().body(serde_json::to_string(&user_data).unwrap_or(String::from("Error while encoding user data as json"))),
        Err(e) => {
            eprintln!("Error while getting user data from sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while fetching companion data, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeFirstMessage {
    first_message: String,
}

#[post("/api/change/firstMessage")]
async fn change_first_message(received: web::Json<ChangeFirstMessage>) -> HttpResponse {
    match Database::change_first_message(&received.first_message) {
        Ok(_) => HttpResponse::Ok().body("Changed first message"),
        Err(e) => {
            eprintln!("Error while changing companion's first message in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing companion's first message, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeCompanionName {
    companion_name: String,
}

#[post("/api/change/companionName")]
async fn change_companion_name(received: web::Json<ChangeCompanionName>) -> HttpResponse {
    match Database::change_companion_name(&received.companion_name) {
        Ok(_) => HttpResponse::Ok().body("Changed companion name"),
        Err(e) => {
            eprintln!("Error while changing companion name in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing companion name, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeUsername {
    user_name: String,
}

#[post("/api/change/userName")]
async fn change_user_name(received: web::Json<ChangeUsername>) -> HttpResponse {
    match Database::change_username(&received.user_name) {
        Ok(_) => HttpResponse::Ok().body("Changed user name"),
        Err(e) => {
            eprintln!("Error while changing username in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing username, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeCompanionPersona {
    companion_persona: String,
}

#[post("/api/change/companionPersona")]
async fn change_companion_persona(received: web::Json<ChangeCompanionPersona>) -> HttpResponse {
    match Database::change_companion_persona(&received.companion_persona) {
        Ok(_) => HttpResponse::Ok().body("Changed companion persona"),
        Err(e) => {
            eprintln!("Error while changing companion persona in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing companion persona, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeCompanionExampleDialogue {
    example_dialogue: String,
}

#[post("/api/change/companionExampleDialogue")]
async fn change_companion_example_dialogue(received: web::Json<ChangeCompanionExampleDialogue>) -> HttpResponse {
    match Database::change_companion_example_dialogue(&received.example_dialogue) {
        Ok(_) => HttpResponse::Ok().body("Changed companion example dialogue"),
        Err(e) => {
            eprintln!("Error while changing companion example dialogue in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing companion example dialogue, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeUserPersona {
    user_persona: String,
}

#[post("/api/change/userPersona")]
async fn change_user_persona(received: web::Json<ChangeUserPersona>) -> HttpResponse {
    match Database::change_user_persona(&received.user_persona) {
        Ok(_) => HttpResponse::Ok().body("Changed user persona"),
        Err(e) => {
            eprintln!("Error while changing user persona in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing user persona, check logs for more information")
        },
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ChangeCompanionData {
    id: u32,
    name: String,
    persona: String,
    example_dialogue: String,
    first_message: String,
    long_term_mem: u32,
    short_term_mem: u32,
    roleplay: bool,
}

#[post("/api/change/companionData")]
async fn change_companion_data(received: web::Json<ChangeCompanionData>) -> HttpResponse {
    match Database::change_companion(&received.name, &received.persona, &received.example_dialogue, &received.first_message, received.long_term_mem, received.short_term_mem, received.roleplay) {
        Ok(_) => HttpResponse::Ok().body("Data of your ai companion has been changed"),
        Err(e) => {
            eprintln!("Error while changing companion data in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing companion data, check logs for more information")
        },
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ChangeUserData {
    id: u32,
    name: String,
    persona: String,
}

#[post("/api/change/userData")]
async fn change_user_data(received: web::Json<ChangeUserData>) -> HttpResponse {
    match Database::change_user(&received.name, &received.persona) {
        Ok(_) => HttpResponse::Ok().body("Data of user has been changed"),
        Err(e) => {
            eprintln!("Error while changing user data in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing user data, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct AddData {
    text: String,
}

#[post("/api/addData")]
async fn add_custom_data(received: web::Json<AddData>) -> HttpResponse {
    match VectorDatabase::connect() {
        Ok(vdb) => {
            match vdb.add_entry(&(received.text.to_string()+"\n")) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error while adding custom data to long-term memory: {}", e);
                    return HttpResponse::InternalServerError().body("Error while adding data to AI long term memory, check logs for more information");
                },
            };
            HttpResponse::Ok().body("Added custom data to AI long term memory")
        },
        Err(_) => HttpResponse::InternalServerError().body("Error while adding data to AI long term memory"),
    }
}

#[get("/api/erase/longTermMemory")]
async fn erase_longterm_mem() -> HttpResponse {
    match VectorDatabase::connect() {
        Ok(vdb) => {
            match vdb.erase_memory() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error while erasing data from long-term memory: {}", e);
                    return HttpResponse::InternalServerError().body("Error while erasing long-term memory, check logs for more information");
                }
            };
            HttpResponse::Ok().body("Erased AI's long term memory")
        },
        Err(_) => HttpResponse::InternalServerError().body("Error while connecting with AI long term memory"),
    }
}

#[derive(Deserialize)]
struct ChangeMemory {
    limit: u32,
}

#[post("/api/change/longTermMemory")]
async fn change_long_term_mem(received: web::Json<ChangeMemory>) -> HttpResponse {
    match Database::change_long_term_memory(received.limit) {
        Ok(_) => HttpResponse::Ok().body("Changed long term memory limit for ai"),
        Err(e) => {
            eprintln!("Error while changing long-term memory limit in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing long-term memory limit, check logs for more information")
        }
    }
}

#[post("/api/change/shortTermMemory")]
async fn change_short_term_mem(received: web::Json<ChangeMemory>) -> HttpResponse {
    match Database::change_short_term_memory(received.limit) {
        Ok(_) => HttpResponse::Ok().body("Changed short term memory limit for ai"),
        Err(e) => {
            eprintln!("Error while changing short-term memory limit in sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while changing short-term memory limit, check logs for more information")
        },
    }
}

#[derive(Deserialize)]
struct ChangeRoleplay {
    enable: bool,
}

#[post("/api/change/roleplay")]
async fn change_roleplay(received: web::Json<ChangeRoleplay>) -> HttpResponse {
    match Database::disable_enable_roleplay(received.enable) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while enabling/disabling roleplay in sqlite database: {}", e);
            return HttpResponse::InternalServerError().body("Error while enabling/disabling roleplay, check logs for more information");
        },
    };
    if received.enable {
        HttpResponse::Ok().body("Enabled roleplay")
    } else {
        HttpResponse::Ok().body("Disabled roleplay")
    }
}

// works with https://zoltanai.github.io/character-editor/
// and with https://github.com/Hukasx0/aichar
#[derive(Serialize, Deserialize)]
struct CharacterJson {
    name: String,
    description: String,
    first_mes: String,
    mes_example: String,
}

#[post("/api/import/characterJson")]
async fn import_character_json(received: web::Json<CharacterJson>) -> HttpResponse {
    match Database::import_companion(&received.name, &received.description, &received.mes_example, &received.first_mes) {
        Ok(_) => HttpResponse::Ok().body("Data of your ai companion has been changed"),
        Err(e) => {
            eprintln!("Error while importing character via json to sqlite database: {}", e);
            HttpResponse::InternalServerError().body("Error while importing character data via json, check logs for more information")
        },
    }
}

// works with https://zoltanai.github.io/character-editor/
// and with https://github.com/Hukasx0/aichar
#[derive(Deserialize)]
struct CharacterCard {
    name: String,
    description: String,
    first_mes: String,
    mes_example: String,
}

#[post("/api/import/characterCard")]
        // curl -X POST -H "Content-Type: image/png" -T card.png http://localhost:3000/api/import/characterCard
async fn import_character_card(mut received: actix_web::web::Payload) -> HttpResponse {
    let mut data = web::BytesMut::new();
    while let Some(chunk) = received.next().await {
        let d = chunk.unwrap();
        data.extend_from_slice(&d);
    }
    let text_chunk_start = data.windows(9).position(|window| window == b"tEXtchara").expect("Looks like this image does not contain character data");
    let text_chunk_end = data.windows(4).rposition(|window| window == b"IEND").expect("Looks like this image does not contain character data");
    let character_base64 = &data[text_chunk_start + 10..text_chunk_end - 8];
    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let character_bytes = match engine.decode(character_base64) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error while decoding base64 character data from character card: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    let character_text: &str = match std::str::from_utf8(&character_bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error while parsing decoded base64 bytes to utf8 string: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    let character_data: CharacterCard = serde_json::from_str(character_text).expect("Your image file does not contain correct json data");
    match Database::import_companion(&character_data.name, &character_data.description, &character_data.mes_example, &character_data.first_mes) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while importing companion data via character card: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
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
            eprintln!("Error while changing companion avatar using character card: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing character card, check logs for more information");
        }
    };
    HttpResponse::Ok().body("Data of your ai companion has been changed")
}

#[post("/api/change/companionAvatar")]
async fn change_companion_avatar(mut received: actix_web::web::Payload) -> HttpResponse {
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
    HttpResponse::Ok().body("Changed avatar of your ai companion")
}

#[derive(Deserialize, Serialize)]
struct MessagesJson {
    messages: Vec<MessageImport>,
}

#[derive(Deserialize, Serialize)]
struct MessageImport {
    ai: bool,
    text: String,
}

#[post("/api/import/messagesJson")]
async fn import_messages_json(received: web::Json<MessagesJson>) -> HttpResponse {
    let mut messages_iter = received.messages.iter();
    for message in messages_iter.to_owned() {
        match Database::add_message(&message.text, message.ai) {
            Ok(_) => {},
            Err(e) => eprintln!("Error while adding message to database/short-term memory: {}", e),
        };
    }
    let vector = match VectorDatabase::connect() {
        Ok(vd) => vd,
        Err(e) => {
            eprintln!("Error while connecting to tantivy: {}", e);
            return HttpResponse::InternalServerError().body("Error while importing messages to long-term memory, check logs for more information");
        }
    };
    while let Some(msg1) = messages_iter.next() {
        if let Some(msg2) = messages_iter.next() {
            match vector.add_entry(&format!("{}: {}\n{}: {}\n", if msg1.ai {"{{char}}"} else {"{{user}}"}, msg1.text, if msg2.ai {"{{char}}"} else {"{{user}}"}, msg2.text)) {
                Ok(_) => {},
                Err(e) => eprintln!("Error while importing message to long-term memory: {}", e),
            };
        }
    }
    HttpResponse::Ok().body("Imported messages to your ai companion memory")
}

#[get("/api/messagesJson")]
async fn get_messages_json() -> HttpResponse {
    let database_messages = match Database::get_messages() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error while fetching messages as json: {}", e);
            return HttpResponse::InternalServerError().body("Error while fetching messages as json, check logs for more information");
        }
    };
    let messages: MessagesJson = MessagesJson { messages: database_messages.iter().map(|message|
        MessageImport {
            ai: match message.ai.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!(),
            },
            text: message.text.clone(),
        }
    ).collect(), };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&messages).unwrap_or(String::from("Error while encoding messages as json")))
}

#[get("/api/characterJson")]
async fn get_character_json() -> HttpResponse {
    let companion_data = match Database::get_companion_data() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error while fetching companion data: {}", e);
            return HttpResponse::InternalServerError().body("Error while fetching companion data as json, check logs for more information");
        }
    };
    let character_data: CharacterJson = CharacterJson {
        name: companion_data.name,
        description: companion_data.persona,
        first_mes: companion_data.first_message,
        mes_example: companion_data.example_dialogue,
    };
    HttpResponse::Ok().body(serde_json::to_string_pretty(&character_data).unwrap_or(String::from("Error while encoding companion data as json")))
}


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
            .service(companion_default_avatar)
            .service(project_logo)
            .service(test_prompt)
            .service(get_messages)
            .service(clear_messages)
            .service(rm_message)
            .service(change_first_message)
            .service(change_companion_name)
            .service(change_companion_persona)
            .service(change_companion_example_dialogue)
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
            .service(change_companion_avatar)
            .service(import_character_json)
            .service(import_character_card)
            .service(import_messages_json)
            .service(get_messages_json)
            .service(get_character_json)
    })
    .bind((hostname, port))?
    .run()
    .await
}
