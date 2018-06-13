#[macro_use] extern crate diesel;
extern crate dotenv;

mod schema;
mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_player<'a>(conn: &SqliteConnection, nickname: &'a str) -> usize {
    use schema::players;

    let new_player = Player {
        nickname: nickname.to_string(),
    };

    diesel::insert_into(players::table)
        .values(&new_player)
        .execute(conn)
        .expect("Error saving new player")
}

pub fn get_players(conn: &SqliteConnection) -> Vec<Player> {
    use schema::players::dsl::*;

    let results = players
        .load::<Player>(conn)
        .expect("Error loading players");

    results
}

pub fn find_player(conn: &SqliteConnection, player_name: &str) -> Option<Player> {
    use schema::players::dsl::*;

    let results = players
        .filter(nickname.eq(player_name))
        .load::<Player>(conn)
        .expect("Error loading players");

    if results.len() == 1 {
        return Some(results[0].clone());
    }

    None
}
