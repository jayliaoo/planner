use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::response::Html;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::validate_request::ValidateRequestHeaderLayer;

use crate::handler::jwt::{BaseJwt, Jwt};
use crate::handler::{sprint_handler, task_handler, user_handler};
use crate::repository::get_connection_pool;
use crate::repository::sprint_repository::SprintRepository;
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::task_service::TaskService;

mod handler;
mod model;
mod repository;
mod schema;
mod service;

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();
    let user_repository = Arc::new(UserRepository::new(pool.clone()));
    let sprint_repository = Arc::new(SprintRepository::new(pool.clone()));
    let task_service = Arc::new(TaskService::new(
        Arc::new(TaskRepository::new(pool)),
        user_repository.clone(),
        sprint_repository.clone(),
    ));

    let base_jwt = BaseJwt::new("6d4f5439-e33a-45e1-b914-9bbe18ee7137".to_string());
    let jwt_arc = Arc::new(base_jwt.clone());
    let jwt = Jwt::new(base_jwt);
    let cors_layer = CorsLayer::new()
        .allow_headers(AllowHeaders::mirror_request())
        .allow_methods(AllowMethods::list([
            Method::POST,
            Method::GET,
            Method::PUT,
            Method::DELETE,
        ]))
        .allow_credentials(true)
        .allow_origin(AllowOrigin::list([HeaderValue::from_str(
            "http://localhost:4200",
        )
        .unwrap()]));
    let app = Router::new()
        .route("/login", post(user_handler::login).with_state(jwt_arc))
        .route("/index", get(|| async { Html("Hello World!") }))
        .nest(
            "",
            Router::new()
                .nest(
                    "/user",
                    Router::new()
                        .route("/", post(user_handler::add))
                        .route("/", put(user_handler::edit))
                        .route("/:id", delete(user_handler::delete))
                        .route("/:id", get(user_handler::get))
                        .route("/list", get(user_handler::get_list))
                        .with_state(user_repository),
                )
                .nest(
                    "/sprint",
                    Router::new()
                        .route("/", post(sprint_handler::add))
                        .route("/", put(sprint_handler::edit))
                        .route("/:id", delete(sprint_handler::delete))
                        .route("/:id", get(sprint_handler::get))
                        .route("/list", get(sprint_handler::get_all))
                        .with_state(sprint_repository),
                )
                .nest(
                    "/task",
                    Router::new()
                        .route("/", post(task_handler::add))
                        .route("/", put(task_handler::edit))
                        .route("/:id", delete(task_handler::delete))
                        .route("/:id", get(task_handler::get))
                        .route("/list", get(task_handler::get_list))
                        .with_state(task_service),
                )
                .layer(ValidateRequestHeaderLayer::custom(jwt)),
        )
        .layer(cors_layer);

    axum::Server::bind(&"0.0.0.0:3500".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
