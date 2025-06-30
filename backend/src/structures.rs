use std::vec::Vec;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub platforms: Vec<String>,
    pub status: String
}

#[derive(Debug, Clone, Deserialize, FromRow, Serialize)]
pub struct NewGame {
    pub name: String,
    pub platforms: Vec<String>,
    pub status: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewGameResponse {
    pub message: String,
    pub game: Game
}

#[derive(Serialize, Deserialize)]
pub struct ModifyGameResponse {
    pub message: String,
    pub game: Game
}

#[derive(Debug, Clone, Deserialize, FromRow, Serialize)]
pub struct DeletedGameMessageResponse {
    pub message: String
}