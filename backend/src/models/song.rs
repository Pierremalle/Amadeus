use serde::{Deserialize, Serialize};
use surrealdb::dbs::node::Timestamp;
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
    timestamp: Timestamp,
    name: String,
    bpm: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: RecordId,
    timestamp: Timestamp,
    name: String,
    bpm: f32,
}