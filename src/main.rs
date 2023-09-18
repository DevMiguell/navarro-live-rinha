use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, extract::{State, Path}, Json,
};
use tokio::sync::RwLock;
use std::collections::HashMap;
use time::{macros::date, Date};
use uuid::Uuid;
use std::sync::Arc;
use serde::Serialize;
use serde::Deserialize;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

type HashMapState = HashMap<Uuid, Person>;

type AppState = Arc<RwLock<HashMapState>>;

#[tokio::main]
async fn main() {
    let mut people: HashMapState = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Miguel"),
        nick: String::from("DevMiguel"),
        birth_date: date!(2000 - 07 - 24),
        stack: None, // vec!["Rust".to_string(), "Go".to_string()]
    };  

    people.insert(person.id, person);


    let app_state = Arc::new(RwLock::new(people));

    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", post(create_person))
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
    match people.read ().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None =>    Err(StatusCode::NOT_FOUND),   
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>
) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let person  = Person {
        id,
        name: new_person.name,
        nick: new_person.nick,
        birth_date: new_person.birth_date,
        stack: new_person.stack
    };

    people.write().await.insert(id, person.clone());
    (StatusCode::OK, Json(person))
}

async fn count_people(
    State(people): State<AppState>
) -> impl IntoResponse {
    let count = people.read().await.len();

    (StatusCode::OK, Json(count))
}
