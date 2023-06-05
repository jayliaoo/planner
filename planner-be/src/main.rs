use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::Router;
use axum::routing::{delete, get, post, put};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::handler::{sprint_handler, task_handler, user_handler};
use crate::repository::get_connection_pool;
use crate::repository::sprint_repository::SprintRepository;
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;

mod handler;
mod repository;
mod schema;
mod model;

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();
    let user_repository = Arc::new(UserRepository::new(&pool));
    let sprint_repository = Arc::new(SprintRepository::new(&pool));
    let task_repository = Arc::new(TaskRepository::new(&pool));
    let cors_layer = CorsLayer::new()
        .allow_headers(AllowHeaders::mirror_request())
        .allow_methods(AllowMethods::list([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::DELETE,
        ]))
        .allow_credentials(true)
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_str("http://localhost:3000").unwrap(),
            HeaderValue::from_str("http://localhost:8080").unwrap(),
        ]));
    let app = Router::new()
        .route("/login", post(user_handler::login))
        .nest("/user", Router::new()
            .route("/", post(user_handler::add))
            .route("/", put(user_handler::edit))
            .route("/:id", delete(user_handler::delete))
            .route("/:id", get(user_handler::get))
            .route("/list", get(user_handler::get_all))
            .with_state(user_repository))
        .nest("/sprint", Router::new()
            .route("/", post(sprint_handler::add))
            .route("/", put(sprint_handler::edit))
            .route("/:id", delete(sprint_handler::delete))
            .route("/:id", get(sprint_handler::get))
            .route("/list", get(sprint_handler::get_all))
            .with_state(sprint_repository))
        .nest("/task", Router::new()
            .route("/", post(task_handler::add))
            .route("/", put(task_handler::edit))
            .route("/:id", delete(task_handler::delete))
            .route("/:id", get(task_handler::get))
            .route("/list", get(task_handler::get_all))
            .with_state(task_repository))
        .layer(cors_layer);

    axum::Server::bind(&"0.0.0.0:2345".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
