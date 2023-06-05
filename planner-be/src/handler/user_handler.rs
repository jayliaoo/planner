use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::handler::jwt::new_token;
use crate::model::entity::User;
use crate::model::MyResult;
use crate::model::param::{AddUserParam, LoginParam};
use crate::repository::user_repository::UserRepository;

static USERNAME: &str = "admin";
static PASSWORD: &str = "fU0CtFrMOgYFFJVn";

pub(crate) async fn login(Json(payload): Json<LoginParam>) -> Json<MyResult<String>> {
    let result = if payload.username == USERNAME && payload.password == PASSWORD {
        MyResult::success(Some(new_token()))
    } else {
        MyResult::fail(403, "username or password is wrong.")
    };
    Json(result)
}

pub(crate) async fn add<'a>(State(repository): State<Arc<UserRepository<'a>>>, Json(payload): Json<AddUserParam>) -> Json<MyResult<i32>> {
    let id = repository.insert(&payload).unwrap();
    Json(MyResult::success(Some(id)))
}

pub(crate) async fn edit<'a>(State(repository): State<Arc<UserRepository<'a>>>, Json(payload): Json<User>) -> Json<MyResult<()>> {
    repository.update(&payload).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn delete<'a>(State(repository): State<Arc<UserRepository<'a>>>, Path(id): Path<i32>) -> Json<MyResult<()>> {
    repository.delete(id).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn get<'a>(State(repository): State<Arc<UserRepository<'a>>>, Path(id): Path<i32>) -> Json<MyResult<User>> {
    let user = repository.get(id).unwrap();
    Json(MyResult::success(Some(user)))
}

pub(crate) async fn get_all<'a>(State(repository): State<Arc<UserRepository<'a>>>) -> Json<MyResult<Vec<User>>> {
    let users = repository.get_all().unwrap();
    Json(MyResult::success(Some(users)))
}
