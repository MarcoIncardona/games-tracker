use axum::{extract::State, routing::get, Json, Router};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod mock;

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
    .route("/games", get(get_games).post(create_games))
    .with_state(pool.clone());  

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is running on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_games(State(pool): State<PgPool>) -> Json<Vec<mock::Game>> {
    let games = sqlx::query_as::<_, mock::Game>("SELECT * FROM games")
        .fetch_all(&pool)
        .await
        .expect("Errore nel recupero dei giochi");
    println!("Giochi trovati: {}", games.len());


    Json(games)
}

async fn create_games(Json(payload): Json<mock::NewGame>) -> Json<mock::Game> {
    let game = mock::Game {
        id: 3,
        name: payload.name,
        platforms: payload.platforms,
        status: payload.status
    }; 

    println!("{:?} è stato aggiunto con successo!", &game.name); 
    Json(game)
}