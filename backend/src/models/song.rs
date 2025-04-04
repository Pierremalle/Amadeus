use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
    pub timestamp: String,
    name: String,
    bpm: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: RecordId,
    timestamp: String,
    name: String,
    bpm: f32,
}
