use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct SongData {
    pub timestamp: String,
    pub name: String,
    pub path: String,
    pub bpm: f32,
    pub duration: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: RecordId,
    timestamp: String,
    name: String,
    path: String,
    bpm: f32,
    duration: f32,
}