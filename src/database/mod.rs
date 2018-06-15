pub mod models;

mod schema;

extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use self::diesel::sqlite::SqliteConnection;
use self::dotenv::dotenv;
use self::models::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_player<'a>(conn: &SqliteConnection, new_player: &Player) -> usize {
    use self::schema::players;

    diesel::insert_into(players::table)
        .values(new_player)
        .execute(conn)
        .expect("Error saving new player")
}

pub fn get_players(conn: &SqliteConnection) -> Vec<Player> {
    use self::schema::players::dsl::*;

    let results = players.load::<Player>(conn).expect("Error loading players");

    results
}

pub fn find_player(conn: &SqliteConnection, player_name: &str) -> Option<Player> {
    use self::schema::players::dsl::*;

    let results = players
        .filter(nickname.eq(player_name))
        .load::<Player>(conn)
        .expect("Error loading players");

    if results.len() == 1 {
        return Some(results[0].clone());
    }

    None
}
