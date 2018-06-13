extern crate database;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate failure_derive;
extern crate failure;

extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder, Path, http, HttpResponse, Json, Result, error};

#[derive(Deserialize)]
struct Player {
    nickname: String,
}

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

fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page"
}

fn hello(path: Path<String>) -> impl Responder {
    format!("Hello {}!", *path)
}

fn add_player(player_info: Json<Player>) -> Result<String, MyError> {
    let connection = database::establish_connection();

    if let Some(_) = database::find_player(&connection, &player_info.nickname) {
        return Err(MyError{name: "Already exists"});
    }
    database::create_player(&connection, &player_info.nickname);
    Ok(String::from("Player added"))
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.method(http::Method::GET).with(index))
            .resource("/hello/{name}", |r| r.method(http::Method::GET).with(hello))
            .resource("/add_player", |r| r.method(http::Method::POST).with(add_player))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}
