#[derive(Deserialize)]
struct CharacterCard {
    name: String,
    description: String,
    first_mes: String,
    mes_example: String,
}


fn load_character_json(json: &str) -> PyResult<CharacterClass> {
    let char_data: LoadCharacterClass = serde_json::from_str(json).expect("Error while parsing json string");
    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

fn load_character_card(bytes: &[u8]) -> PyResult<CharacterClass> {
    let decoder = png::Decoder::new(Cursor::new(bytes));
    let reader = decoder.read_info().map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read PNG info: {}", e)))?;
    let character_base64_option: Option<String> = reader.info().uncompressed_latin1_text.iter()
        .filter(|text_chunk| text_chunk.keyword == "chara")
        .map(|text_chunk| text_chunk.text.clone())
        .next();
    let character_base64: String = match character_base64_option {
        Some(v) => v,
        None => {
            let text_chunk_start = bytes.windows(9).position(|window| window == b"tEXtchara").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No tEXt chunk with name 'chara' found"))?;
            let text_chunk_end = bytes.windows(4).rposition(|window| window == b"IEND").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No tEXt chunk with name 'chara' found"))?;
            String::from_utf8_lossy(&bytes[text_chunk_start + 10..text_chunk_end - 8]).to_string()
        }
    };
    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let character_bytes = match engine.decode(character_base64) {
        Ok(b) => b,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error while decoding base64 character data from character card: {:?}",
                e
            )));
        }
    };
    let character_text: &str = match std::str::from_utf8(&character_bytes) {
        Ok(s) => s,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error while parsing decoded base64 bytes to utf8 string: {:?}",
                e
            )));
        }
    };
    let char_data: LoadCharacterClass =
        serde_json::from_str(character_text).expect("Your image file does not contain correct json data");

    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

