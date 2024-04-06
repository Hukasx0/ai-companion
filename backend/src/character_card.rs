use serde::{Serialize, Deserialize};
use base64::{Engine, engine::GeneralPurpose, engine::GeneralPurposeConfig, alphabet::STANDARD};
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct CharacterCard {
    pub name: String,
    pub description: String,
    pub first_mes: String,
    pub mes_example: String,
}

impl CharacterCard {
    pub fn load_character_card(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        let reader = decoder.read_info()?;
        let character_base64_option: Option<String> = reader.info().uncompressed_latin1_text.iter()
            .filter(|text_chunk| text_chunk.keyword == "chara")
            .map(|text_chunk| text_chunk.text.clone())
            .next();
        let character_base64: String = match character_base64_option {
            Some(v) => v,
            None => {
                let text_chunk_start = bytes.windows(9).position(|window| window == b"tEXtchara").ok_or_else(|| Box::<dyn std::error::Error>::from("No tEXt chunk with name 'chara' found"))?;
                let text_chunk_end = bytes.windows(4).rposition(|window| window == b"IEND").ok_or_else(|| Box::<dyn std::error::Error>::from("No tEXt chunk with name 'chara' found"))?;
                String::from_utf8_lossy(&bytes[text_chunk_start + 10..text_chunk_end - 8]).to_string()
            }
        };
        let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
        let character_bytes = match engine.decode(character_base64) {
            Ok(b) => b,
            Err(e) => {
                return Err(Box::<dyn std::error::Error>::from(format!("Error while decoding base64 character data from character card: {:?}", e)));
            }
        };
        let character_text: &str = match std::str::from_utf8(&character_bytes) {
            Ok(s) => s,
            Err(e) => {
                return Err(Box::<dyn std::error::Error>::from(format!("Error while parsing decoded base64 bytes to utf8 string: {:?}", e)));
            }
        };
        let char_data: CharacterCard = serde_json::from_str(character_text).expect("Your image file does not contain correct json data");
        Ok(char_data)
    }
}
