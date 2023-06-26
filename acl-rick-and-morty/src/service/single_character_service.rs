use crate::model::{error_message::CustomError, rick_and_morty_single_character::Character};
use anyhow::Result;
use reqwest::{blocking::get};

const URL_CHARACTER: &'static str = "https://rickandmortyapi.com/api/character/";

pub fn rick_and_morty_call_api_character(character_id: String) -> Character {
    println!("service ID {}", character_id);

    let url = URL_CHARACTER;

    let response = call_api_backend_sync_response_json(character_id, url.to_string());
    println!("antes de transformar el Character");
    println!("{:?}", response);
    println!("posterior al Character");
    return response;
}

fn call_api_backend_sync_response_json(character_id: String, url: String) -> Character {
    let url_string = format!("{}{}", url, character_id);
    println!("URL : {}", url_string);

    let response = std::thread::spawn(move || {
        let resp = reqwest::blocking::get(&url_string).unwrap();
        let json_value: Character = serde_json::from_str(&resp.text().unwrap_or_default()).unwrap();
        return json_value;
    })
    .join()
    .unwrap();

    return response;
}

pub fn rick_and_morty_call_api_character_two(
    character_id: String,
) -> Result<Character, CustomError> {
    let mut status_code = 404;
    let mut status_message: String = "NOT FOUND".to_string();
    let url_string = format!("{}{}", URL_CHARACTER, character_id);

    let response = std::thread::spawn(move || {
        let resp = get(&url_string).map_err(|err| CustomError {
            code: 500,
            message: err.to_string(),
        })?;

        fun_status_code_message(&mut status_code, &mut status_message, &resp);

        let json_value: Character = resp.json().map_err(|_err| make_custom_error(status_code, &mut status_message))?;

        Ok(json_value)
    })
    .join()
    .unwrap();

    response
}

fn fun_status_code_message(status_code: &mut u16, status_message: &mut String, resp: &reqwest::blocking::Response) {
    *status_code = resp.status().as_u16();
    *status_message = resp.status().canonical_reason().unwrap_or("Unknown status").to_string();
}

fn make_custom_error(status_code: u16, status_message: &mut String) -> CustomError {
    CustomError {
        code: status_code,
        message: status_message.to_string(),
    }
}
