use std::io::Write;
use chrono::{DateTime, Local};

use crate::database::{Database, NewMessage, Message, ConfigView, UserView, CompanionView, PromptTemplate, Device};
use crate::dialogue_tuning::DialogueTuning;
use crate::long_term_mem::LongTermMem;

pub fn prompt(prompt: &str) -> Result<String, std::io::Error> {
    let long_term_memory = match LongTermMem::connect() {
        Ok(ltm) => ltm,
        Err(e) => {
            eprintln!("Error while connecting to tantivy: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while connecting to tantivy"));
        }
    };
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("* at %A %d.%m.%Y %H:%M *\n").to_string();
    let config: ConfigView = match Database::get_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error while getting config: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while getting config"));
        }
    };
    let user: UserView = match Database::get_user_data() {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error while getting user data: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while getting user data"));
        }
    };
    let companion: CompanionView = match Database::get_companion_data() {
        Ok(companion) => companion,
        Err(e) => {
            eprintln!("Error while getting companion data: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while getting companion data"));
        }
    };

    let llama_model_params = {
        let mut params = llm::ModelParameters::default();
        if config.device == Device::GPU || config.device == Device::Metal {
            params.use_gpu = true;
            params.gpu_layers = Some(20);
        } else {
            params.use_gpu = false;
            params.gpu_layers = None;
        }
        params
    };
    
    let llama = llm::load(
        std::path::Path::new(&config.llm_model_path),
        llm::TokenizerSource::Embedded,
        llama_model_params,
        llm::load_progress_callback_stdout,
    );
   
    let llama = match llama {
        Ok(llama) => llama,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to load llm model: {}", e.to_string()))),
    };

    let mut session = llama.start_session(Default::default());
    println!("Generating ai response...");
    let mut base_prompt: String;
    let mut rp: &str = "";
    let mut tuned_dialogue: String = String::from("");
    if companion.roleplay {
        rp = "gestures and other non-verbal actions are written between asterisks (for example, *waves hello* or *moves closer*)";
    }
    if companion.dialogue_tuning {
        match DialogueTuning::get_random_dialogue() {
            Ok(dialogue) => {
                tuned_dialogue = format!("{}: {}\n{}: {}", &user.name, &dialogue.user_msg, &companion.name, &dialogue.ai_msg);
            }
            Err(_) => {}
        };
    }
    if config.prompt_template == PromptTemplate::Default {
        base_prompt = 
        format!("Text transcript of a conversation between {} and {}. {}\n{}'s Persona: {}\n{}'s Persona: {}\n<START>\n{}\n<START>\n{}\n<START>\n", 
                                            user.name, companion.name, rp, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), &tuned_dialogue);
    }
    else if config.prompt_template == PromptTemplate::Llama2 {
        base_prompt = 
        format!("<<SYS>>\nYou are {}, {}\nyou are talking with {}, {} is {}\n{}\n[INST]\n{}\n{}\n[/INST]",
                companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), user.name, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), rp, companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), &tuned_dialogue);
    }
    else {
        base_prompt = 
        format!("<s>[INST]Text transcript of a conversation between {} and {}. {}\n{}'s Persona: {}\n{}'s Persona: {}[/INST]\n<s>[INST]\n{}[/INST]\n<s>[INST]\n{}\n[/INST]\n",
        user.name, companion.name, rp, user.name, user.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.name, companion.persona.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), companion.example_dialogue.replace("{{char}}", &companion.name).replace("{{user}}", &user.name), &tuned_dialogue);
    }
    if companion.long_term_mem > 0 {
        let long_term_memory_entries: Vec<String> = match long_term_memory.get_matches(prompt, companion.long_term_mem) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error while getting long term memory entries: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while getting long term memory entries"));
            }
        };
        for entry in long_term_memory_entries {
            if config.prompt_template == PromptTemplate::Llama2 {
                base_prompt += &format!("[INST]{}[/INST]\n", entry).replace("{{char}}", &companion.name).replace("{{user}}", &user.name);
            }
            else if config.prompt_template == PromptTemplate::Mistral {
                base_prompt += &format!("<s>[INST]{}[/INST]\n", entry).replace("{{char}}", &companion.name).replace("{{user}}", &user.name);
            }
            else {
                base_prompt += &entry.replace("{{char}}", &companion.name).replace("{{user}}", &user.name);
            }
        }
    }
    let short_term_memory_entries: Vec<Message> = match Database::get_x_messages(
        if companion.short_term_mem > 0 { companion.short_term_mem } else { 1 }, 0) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error while getting short term memory entries: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while getting short term memory entries"));
        }
    };
    for message in short_term_memory_entries {
        let prefix = if message.ai { &companion.name } else { &user.name };
        let text = message.content;
        let formatted_message = format!("{}: {}\n", prefix, text);
        if config.prompt_template == PromptTemplate::Llama2 {
            if !message.ai {
                base_prompt += &format!("[INST]{}", formatted_message);
            }
            else {
                base_prompt += &format!("{}[/INST]\n", formatted_message);
            }
        }
        else if config.prompt_template == PromptTemplate::Mistral {
            if !message.ai {
                base_prompt += &format!("<s>[INST]{}", formatted_message);
            }
            else {
                base_prompt += &format!("{}[/INST]\n", formatted_message);
            }
        }
        else {
            base_prompt += &formatted_message;
        }
    }
    let mut end_of_generation = String::new();
    let eog = format!("\n{}:", user.name);
    let res = session.infer::<std::convert::Infallible>(
        llama.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: llm::Prompt::Text(&format!("{}{}: ", &base_prompt, companion.name)),
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
                    //  x = x.clone()+&token;
                    end_of_generation.push_str(&token);
                    print!("{token}");
                    if end_of_generation.contains(&eog) || end_of_generation.contains("[/INST]") || end_of_generation.contains("<</SYS>>") ||
                       end_of_generation.contains("[s]") ||
                       end_of_generation.contains(&format!("{}:", &companion.name)) || end_of_generation.contains(&format!("{}:", &user.name)) ||
                       end_of_generation.contains("<|user|>") {
                        return Ok(llm::InferenceFeedback::Halt);          
                    }
                }
                llm::InferenceResponse::EotToken => {}
            }
            std::io::stdout().flush().unwrap();
            Ok(llm::InferenceFeedback::Continue)
        }
    );
    let x: String = end_of_generation.replace(&eog, "").replace("[INST]", "").replace("[/INST]", "").replace("<</SYS>>", "").replace("<s>", "").replace("</s>", "").replace("<|user|>", "");
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    let companion_text = x
    .split(&format!("\n{}: ", &companion.name))
    .next()
    .unwrap_or("");
    match Database::insert_message(NewMessage { ai: true, content: companion_text.to_string() }) {
        Ok(_) => {},
        Err(e) => eprintln!("Error while adding message to database/short-term memory: {}", e),
    };
    match long_term_memory.add_entry(&format!("{}{}: {}\n{}: {}\n", formatted_date, "{{user}}", &prompt, "{{char}}", &companion_text)) {
        Ok(_) => {},
        Err(e) => eprintln!("Error while adding message to long-term memory: {}", e),
    };
    Ok(companion_text.trim_start().to_string())
}