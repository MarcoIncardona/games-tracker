use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod structures;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL non trovato");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("unable to connect to database");

    let app: Router<()> = Router::new()
    .route("/games", get(get_games).post(create_game))
    .with_state(pool.clone());  

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is running on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_games(State(pool): State<PgPool>) -> Json<Vec<structures::Game>> {
    let games = sqlx::query_as::<_, structures::Game>("SELECT * FROM games")
        .fetch_all(&pool)
        .await
        .expect("Errore nel recupero dei giochi");
    println!("Giochi trovati: {}", games.len());


    Json(games)
}

async fn create_game(
    State(pool): State<PgPool>,
    Json(new_game): Json<structures::NewGame>,
) -> Result<Json<structures::NewGameResponse>, (StatusCode, String)> {
    let result = sqlx::query_as::<_, structures::Game>(
        "INSERT INTO games (name, platforms, status)
         VALUES ($1, $2, $3)
         RETURNING id, name, platforms, status",
    )
    .bind(&new_game.name)
    .bind(&new_game.platforms)
    .bind(&new_game.status)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(game) => {
            let response = structures::NewGameResponse {
                message: "Gioco aggiunto con successo".to_string(),
                game,
            };
            Ok(Json(response))
        }
        Err(err) => {
            eprintln!("Errore nel salvataggio: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Errore nel salvataggio".into()))
        }
    }
}