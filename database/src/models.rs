use schema::players;

#[derive(Queryable, Clone, Insertable)]
#[table_name="players"]
pub struct Player {
    pub nickname: String,
}

// #[derive(Insertable)]
// #[table_name="players"]
// pub struct NewPlayer<'a> {
//     pub name: &'a str,
// }
