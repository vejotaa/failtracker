use axum::{
    routing::{get},
    Router,
    Json,
    http::Method,
    extract::State,
};
use serde::Serialize;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use rand::{prelude::IndexedRandom};

// use hyper::Server;

#[derive(Serialize)]
struct Falha {
    id: String,
    timestamp: String,
    nome: String,
}

async fn root() -> &'static str {
    "Failtracker API está rodando"
}

async fn listar_falhas(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<Falha>>, String> {
    let rows = sqlx::query("SELECT id, timestamp, nome FROM falhas ORDER BY timestamp DESC")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let falhas = rows
        .into_iter()
        .map(|row| Falha {
            id: row.try_get("id").unwrap(),
            timestamp: row.try_get("timestamp").unwrap(),
            nome: row.try_get("nome").unwrap(),
        })
        .collect();

    Ok(Json(falhas))
}

async fn criar_falha(State(pool): State<Arc<SqlitePool>>) -> Result<&'static str, String> {
    let id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().to_rfc3339();
    let nome = nome_aleatorio();
    sqlx::query("INSERT INTO falhas (id, timestamp, nome) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&timestamp)
        .bind(&nome)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Falha registrada")
}

async fn limpar_falhas(State(pool): State<Arc<SqlitePool>>) -> Result<&'static str, String> {
    sqlx::query("DELETE FROM falhas")
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Todas as falhas foram apagadas")
}

fn nome_aleatorio() -> String {
const ADJETIVOS: [&str; 50] = [
  "Rápido", "Brilhante", "Sorridente", "Travesso", "Zumbido",
  "Furioso", "Gentil", "Espiritual", "Curioso", "Magnífico",
  "Valente", "Ágil", "Sábio", "Lento", "Radiante",
  "Sombrio", "Encantado", "Feliz", "Misterioso", "Bravo",
  "Alegre", "Doce", "Orgulhoso", "Calmo", "Tempestuoso",
  "Tímido", "Feroz", "Amável", "Veloz", "Sonhador",
  "Cauteloso", "Feliz", "Vibrante", "Intenso", "Desajeitado",
  "Energético", "Sereno", "Ardente", "Zeloso", "Leal",
  "Gracioso", "Brando", "Imponente", "Astuto", "Paciente",
  "Feliz", "Bravo", "Generoso", "Forte", "Luminoso"
];

const CORES: [&str; 50] = [
  "Vermelho", "Azul", "Verde", "Amarelo", "Roxo",
  "Laranja", "Ciano", "Magenta", "Marrom", "Cinza",
  "Rosa", "Turquesa", "Dourado", "Prateado", "Bege",
  "Lavanda", "Pêssego", "Oliva", "Carmesim", "Safira",
  "Esmeralda", "Âmbar", "Cobre", "Chocolate", "Lilás",
  "Verde-limão", "Bordô", "Ciano-piscina", "Índigo", "Mostarda",
  "Púrpura", "Rosa-choque", "Verde-musgo", "Azul-petróleo", "Cinzento",
  "Creme", "Vinho", "Turquesa-escuro", "Bronze", "Magenta-claro",
  "Coral", "Verde-água", "Amarelo-ouro", "Marfim", "Azul-royal",
  "Rosa-bebê", "Verde-floresta", "Cinza-chumbo", "Lavanda-claro", "Cobre-escuro"
];

const ANIMAIS: [&str; 50] = [
  "Tigre", "Panda", "Canguru", "Coruja", "Foca",
  "Lobo", "Girafa", "Elefante", "Leão", "Urso",
  "Raposa", "Coala", "Cavalo", "Golfinho", "Arraia",
  "Corvo", "Gato", "Cachorro", "Tartaruga", "Cervo",
  "Búfalo", "Camaleão", "Doninha", "Ema", "Falcão",
  "Galo", "Hiena", "Jacaré", "Kiwi", "Lêmure",
  "Morcego", "Narval", "Orca", "Pinguim", "Quati",
  "Rena", "Sabiá", "Tamanduá", "Urso-pardo", "Vaca",
  "Xerente", "Zebra", "Avestruz", "Bicho-preguiça", "Cobra",
  "Dromedário", "Enguia", "Flamingo", "Galo-da-serra", "Hipopótamo"
];
    let mut rng = rand::rng();
    let adj = ADJETIVOS.choose(&mut rng).unwrap();
    let cor = CORES.choose(&mut rng).unwrap();
    let animal = ANIMAIS.choose(&mut rng).unwrap();

    format!("{} {} {}", adj, cor, animal)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let pool = Arc::new(SqlitePool::connect("sqlite://./failtracker.db").await?);
    // let pool = Arc::new(SqlitePool::connect("sqlite:///data/failtracker.db").await?);
    let pool = Arc::new(SqlitePool::connect("sqlite://data/failtracker.db").await?);

    // Cria tabela se não existir
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS falhas (
            id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            nome TEXT NOT NULL
        )",
    )
    .execute(&*pool)
    .await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/falhas", get(listar_falhas).post(criar_falha).delete(limpar_falhas))
        .with_state(pool.clone())
        .layer(cors);

    //println!("API rodando em 127.0.0.1:3000");
    println!("API rodando em 0.0.0.0:3000");
    //hyper::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    // // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await?;

    //let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?; // listen on all interfaces

    axum::serve(listener, app).await?;


    Ok(())
}