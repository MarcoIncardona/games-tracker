use std::vec::Vec;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Game {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) platforms: Vec<String>,
    pub(crate) status: String
}

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct NewGame {
    pub(crate) name: String,
    pub(crate) platforms: Vec<String>,
    pub(crate) status: String,
}
