use std::{env, net::TcpListener, process::Command};

use actix_web::{
    http::StatusCode, post, web::Json, App, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder,
};
use dotenv::dotenv;
use toolbox_link_escape::Message;

#[post("/open")]
pub async fn link_open_service(payload: Json<Message>) -> impl Responder {
    let payload = payload.into_inner();
    println!("{payload:?}");
    if payload.code != std::env::var("TLE_SECRET").unwrap_or("TLE_TEST".to_owned()) {
        return HttpResponse::Unauthorized().finish();
    }
    let command = Command::new("xdg-open").arg(payload.link).output();
    if command.is_err() {
        eprintln!("{command:?}");
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponseBuilder::new(StatusCode::OK).finish()
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let res = dotenv();
    if let Err(e) = res {
        eprintln!(".env file could not be opened: {e}")
    }
    println!(".env file read");
    HttpServer::new(move || App::new().service(link_open_service))
        .listen(TcpListener::bind(format!(
            "127.0.0.1:{}",
            env::var("TLE_PORT").unwrap_or("8888".to_owned())
        ))?)?
        .run()
        .await
}
