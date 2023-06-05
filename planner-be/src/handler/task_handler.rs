use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::model::entity::Task;
use crate::model::MyResult;
use crate::model::param::AddTaskParam;
use crate::repository::task_repository::TaskRepository;

pub(crate) async fn add<'a>(State(repository): State<Arc<TaskRepository<'a>>>, Json(payload): Json<AddTaskParam>) -> Json<MyResult<i32>> {
    let id = repository.insert(&payload).unwrap();
    Json(MyResult::success(Some(id)))
}

pub(crate) async fn edit<'a>(State(repository): State<Arc<TaskRepository<'a>>>, Json(payload): Json<Task>) -> Json<MyResult<()>> {
    repository.update(&payload).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn delete<'a>(State(repository): State<Arc<TaskRepository<'a>>>, Path(id): Path<i32>) -> Json<MyResult<()>> {
    repository.delete(id).unwrap();
    Json(MyResult::success(None))
}

pub(crate) async fn get<'a>(State(repository): State<Arc<TaskRepository<'a>>>, Path(id): Path<i32>) -> Json<MyResult<Task>> {
    let task = repository.get(id).unwrap();
    Json(MyResult::success(Some(task)))
}

pub(crate) async fn get_all<'a>(State(repository): State<Arc<TaskRepository<'a>>>) -> Json<MyResult<Vec<Task>>> {
    let tasks = repository.get_all().unwrap();
    Json(MyResult::success(Some(tasks)))
}
