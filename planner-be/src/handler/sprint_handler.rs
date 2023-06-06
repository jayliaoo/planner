use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::model::entity::Sprint;
use crate::model::MyResult;
use crate::model::param::AddSprintParam;
use crate::repository::sprint_repository::SprintRepository;

pub(crate) async fn add(State(repository): State<Arc<SprintRepository>>, Json(payload): Json<AddSprintParam>) -> Json<MyResult<i32>> {
    let id = repository.insert(&payload).unwrap();
    Json(MyResult::success(Some(id)))
}

pub(crate) async fn edit(State(repository): State<Arc<SprintRepository>>, Json(payload): Json<Sprint>) -> Json<MyResult<()>> {
    repository.update(&payload).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn delete(State(repository): State<Arc<SprintRepository>>, Path(id): Path<i32>) -> Json<MyResult<()>> {
    repository.delete(id).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn get(State(repository): State<Arc<SprintRepository>>, Path(id): Path<i32>) -> Json<MyResult<Sprint>> {
    let sprint = repository.get(id).unwrap();
    Json(MyResult::success(Some(sprint)))
}

pub(crate) async fn get_all(State(repository): State<Arc<SprintRepository>>) -> Json<MyResult<Vec<Sprint>>> {
    let sprints = repository.get_all().unwrap();
    Json(MyResult::success(Some(sprints)))
}
