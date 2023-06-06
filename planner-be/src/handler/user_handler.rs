use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::handler::jwt::BaseJwt;
use crate::model::entity::User;
use crate::model::MyResult;
use crate::model::param::{AddUserParam, LoginParam, UserListParam};
use crate::repository::user_repository::UserRepository;

static USERNAME: &str = "admin";
static PASSWORD: &str = "fU0CtFrMOgYFFJVn";

pub(crate) async fn login(State(jwt): State<Arc<BaseJwt>>, Json(payload): Json<LoginParam>) -> Json<MyResult<String>> {
    let result = if payload.username == USERNAME && payload.password == PASSWORD {
        MyResult::success(Some(jwt.new_token()))
    } else {
        MyResult::fail(403, "username or password is wrong.")
    };
    Json(result)
}

pub(crate) async fn add(State(repository): State<Arc<UserRepository>>, Json(payload): Json<AddUserParam>) -> Json<MyResult<i32>> {
    let id = repository.insert(&payload).unwrap();
    Json(MyResult::success(Some(id)))
}

pub(crate) async fn edit(State(repository): State<Arc<UserRepository>>, Json(payload): Json<User>) -> Json<MyResult<()>> {
    repository.update(&payload).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn delete(State(repository): State<Arc<UserRepository>>, Path(id): Path<i32>) -> Json<MyResult<()>> {
    repository.delete(id).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn get(State(repository): State<Arc<UserRepository>>, Path(id): Path<i32>) -> Json<MyResult<User>> {
    let user = repository.get(id).unwrap();
    Json(MyResult::success(Some(user)))
}

pub(crate) async fn get_list(State(repository): State<Arc<UserRepository>>, Query(param):Query<UserListParam>) -> Json<MyResult<Vec<User>>> {
    let users = repository.get_list(param).unwrap();
    Json(MyResult::success(Some(users)))
}
