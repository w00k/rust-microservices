use actix_web::{ App, HttpServer};

pub mod controller;
pub mod service;
pub mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("ACL Starting ");

    let addrs = ("127.0.0.1", 8080);

    HttpServer::new(move || {
        App::new()
        .service(controller::hello_world::hello)
        .service(controller::single_character::single_character_controller)
        .service(controller::single_character::single_character_controller_two)
    })
    .bind(addrs)?
    .run()
    .await
}
