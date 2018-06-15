use database::schema::players;

#[derive(Queryable, Clone, Insertable, Deserialize)]
#[table_name="players"]
pub struct Player {
    pub nickname: String,
    pub name: String,
    pub lastname: String,
}
