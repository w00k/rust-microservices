use actix_web::{get, HttpResponse, Responder, web};

use crate::service;

#[get("/single-character/{character_id}")]
async fn single_character_controller(path: web::Path<String>) -> impl Responder {
    let character_id = path.into_inner();
    let response = service::rick_and_morty_api_character(character_id);
    HttpResponse::Ok().json(response)
}

#[get("/single-character-two/{character_id}")]
async fn single_character_controller_two(path: web::Path<String>) -> impl Responder {
    let character_id = path.into_inner();
    let response = service::rick_and_morty_api_character_two(character_id);
    
    if response.is_err() {
        println!("error");
        let message_error = response.err().unwrap();
        return HttpResponse::build(message_error.create_status_code()).json(message_error);

    } 

    return HttpResponse::Ok().json(response.unwrap());
}