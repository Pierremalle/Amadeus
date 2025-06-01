use crate::model::song::{Song, SongData};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonData {
    name: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    instruments: Vec<String>,
    compositions: Vec<SongData>,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: RecordId,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    instruments: Vec<String>,
    compositions: Vec<Song>,
}
