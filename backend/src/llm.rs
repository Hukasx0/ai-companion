pub fn prompt(prompt: &str) -> Result<String, std::io::Error> {
    let long_term_memory = match LongTermMem::connect() {
        Ok(ltm) => ltm,
        Err(e) => {
            eprintln!("Error while connecting to tantivy: {}", e);
        }
    };
    let config: Config = Database::get_config();
    let user: UserView = Database::get_user_data();
    let companion: CompanionView = Database::get_companion_data();
    
}