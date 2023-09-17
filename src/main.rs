use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, extract::{State, Path}, Json,
};
use std::collections::HashMap;
use time::{macros::date, Date};
use uuid::Uuid;
use std::sync::Arc;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stack: Vec<String>,
}

type HashMapState = HashMap<Uuid, Person>;

type AppState = Arc<HashMapState>;

#[tokio::main]
async fn main() {
    let mut people: HashMapState = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Miguel"),
        nick: String::from("DevMiguel"),
        birth_date: date!(2000 - 07 - 24),
        stack: vec!["Rust".to_string(), "Go".to_string()],
    };

    people.insert(person.id, person);

    let app_state = Arc::new(people);

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoa", post(create_person))
        .route("/contagem-pessoas", get(count_people))
        .with_state(app_state);
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_people(state: State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Buscar Pessoas") 
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid> 
) -> impl IntoResponse {   
    let person = people.get(&person_id);

    match person {
        Some(person) => Ok(Json(person.clone())),
        None =>    Err(StatusCode::NOT_FOUND),   
    }
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::OK, "Create")
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::OK, "Count")
}
