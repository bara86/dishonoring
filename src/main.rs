extern crate dishonoring;

#[macro_use] extern crate failure_derive;
extern crate failure;

extern crate actix_web;
use actix_web::{server, App, http, HttpResponse, Json, Result, error};

use dishonoring::database;
use database::models::*;

#[derive(Fail, Debug)]
#[fail(display="Error")]
struct MyError {
    name: &'static str
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Conflict().body(self.name)
    }
}

fn add_player(player_info: Json<Player>) -> Result<String, MyError> {
    let connection = database::establish_connection();

    if let Some(_) = database::find_player(&connection, &player_info.nickname) {
        return Err(MyError{name: "Already exists"});
    }

    let new_player = Player {
        nickname: player_info.nickname.clone(),
        name: player_info.name.clone(),
        lastname: player_info.lastname.clone(),
    };

    database::create_player(&connection, &new_player);
    Ok(String::from("Player added"))
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/add_player", |r| r.method(http::Method::POST).with(add_player))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
