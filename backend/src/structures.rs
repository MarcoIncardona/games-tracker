use std::vec::Vec;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub platforms: Vec<String>,
    pub status: String
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct NewGame {
    pub name: String,
    pub platforms: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct NewGameResponse {
    pub message: String,
    pub game: Game
}
