use llm::Model;
use std::io::Write;
use std::fs;
use chrono::{DateTime, Local};
use crate::Database;
use crate::database::{Message, CompanionData, UserData};
use crate::vectordb::VectorDatabase;
use crate::dialogue_tuning::DialogueTuning;

pub fn prompt(text_prompt: &str) -> Result<String, String> {
    let vector = match VectorDatabase::connect() {
        Ok(vd) => vd,
        Err(e) => {
            eprintln!("Error while connecting to tantivy: {}", e);
            return Err("Error while generating output message, check logs for more information".to_string());
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
    println!("Generating ai response...");
    let companion: CompanionData = match Database::get_companion_data() {
        Ok(cd) => cd,
        Err(e) => {
            eprintln!("Error while getting companion data from sqlite database: {}", e);
            return Err("Error while generating output message, check logs for more information".to_string());
        }
    };
    let user: UserData = match Database::get_user_data() {
        Ok(ud) => ud,
        Err(e) => {
            eprintln!("Error while getting user data from sqlite database: {}", e);
            return Err("Error while generating output message, check logs for more information".to_string());
        }
    };
    let mut base_prompt: String;
    let mut rp: &str = "";
    let mut tuned_dialogue: String = String::from("");
    if companion.roleplay == 1 {
        rp = "gestures and other non-verbal actions are written between asterisks (for example, *waves hello* or *moves closer*)";
    }
    if companion.dialogue_tuning == 1 {
        match DialogueTuning::get_random_dialogue() {
            Ok(dialogue) => {
                tuned_dialogue = format!("{}: {}\n{}: {}", &user.name, &dialogue.user_msg, &companion.name, &dialogue.ai_msg);
            },
            Err(_) => {},
        };
    }
    if is_llama2 {
        base_prompt = 
        format!("<<SYS>>\nYou are {}, {}\nyou are talking with {}, {} is {}\n{}\n[INST]\n{}\n{}\n[/INST]",
                companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), user.name, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), rp, companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), &tuned_dialogue);
    } else {
        base_prompt = 
        format!("Text transcript of a conversation between {} and {}. {}\n{}'s Persona: {}\n{}'s Persona: {}\n<START>\n{}\n<START>\n{}\n<START>\n", 
                                            user.name, companion.name, rp, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), &tuned_dialogue);
    }
    let abstract_memory: Vec<String> = match vector.get_matches(text_prompt, companion.long_term_mem) {
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
            return Err("Error while generating output message, check logs for more information".to_string());
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
    let x: String = end_of_generation.replace(&eog, "");
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
    match vector.add_entry(&format!("{}{}: {}\n{}: {}\n", formatted_date, "{{user}}", &text_prompt, "{{char}}", &companion_text)) {
        Ok(_) => {},
        Err(e) => eprintln!("Error while adding message to long-term memory: {}", e),
    };
    Ok(companion_text.to_string())
}
