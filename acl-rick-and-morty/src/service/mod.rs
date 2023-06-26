use crate::model::{rick_and_morty_single_character::Character, error_message::CustomError};

mod single_character_service;

pub fn rick_and_morty_api_character(character_id: String) -> Character {
    println!("mod service ID {}", character_id);
    single_character_service::rick_and_morty_call_api_character(character_id)
}

pub fn rick_and_morty_api_character_two(character_id: String) -> Result<Character, CustomError> {
    println!("mod service ID {}", character_id);
    single_character_service::rick_and_morty_call_api_character_two(character_id)
}