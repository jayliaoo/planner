use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::Path;
use axum::http::{header, HeaderValue, Method, Request, Response, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::routing::{delete, get, post, put};
use futures::future::BoxFuture;
use hyper::Body;
use tower_http::auth::AsyncAuthorizeRequest;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeaderLayer};

use crate::handler::sprint_handler::SprintHandler;
use crate::handler::task_handler::TaskHandler;
use crate::handler::user_handler::UserHandler;
use crate::model::entity::{Sprint, Task, User};
use crate::model::param::{AddSprintParam, AddTaskParam, AddUserParam, LoginParam};
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
    let user_handler = Arc::new(UserHandler::new(UserRepository::new(&pool)));
    let mut sprint_handler = Arc::new(SprintHandler::new(SprintRepository::new(&pool)));
    let mut task_handler = Arc::new(TaskHandler::new(TaskRepository::new(&pool)));
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
        .route("/login", post(|Json(payload): Json<LoginParam>| user_handler.login(payload)))
        .route("/user", post(|Json(payload): Json<AddUserParam>| user_handler.add(&payload)))
        .route("/user", put(|Json(payload): Json<User>| user_handler.edit(&payload)))
        .route("/user/:id", delete(|Path(id): Path<i32>| user_handler.delete(id)))
        .route("/user/:id", get(|Path(id): Path<i32>| user_handler.get(id)))
        .route("/user/list", get(|| user_handler.get_all()))
        .route("/sprint", post(|Json(payload): Json<AddSprintParam>| sprint_handler.add(&payload)))
        .route("/sprint", put(|Json(payload): Json<Sprint>| sprint_handler.edit(&payload)))
        .route("/sprint/:id", delete(|Path(id): Path<i32>| sprint_handler.delete(id)))
        .route("/sprint/:id", get(|Path(id): Path<i32>| sprint_handler.get(id)))
        .route("/sprint/list", get(|| sprint_handler.get_all()))
        .route("/task", post(|Json(payload): Json<AddTaskParam>| task_handler.add(&payload)))
        .route("/task", put(|Json(payload): Json<Task>| task_handler.edit(&payload)))
        .route("/task/:id", delete(|Path(id): Path<i32>| task_handler.delete(id)))
        .route("/task/:id", get(|Path(id): Path<i32>| task_handler.get(id)))
        .route("/task/list", get(|| task_handler.get_all()))
        .layer(cors_layer);

    axum::Server::bind(&"0.0.0.0:2345".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
