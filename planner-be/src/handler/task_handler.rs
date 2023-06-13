use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::model::param::{AddTaskParam, EditTaskParam, TaskListParam};
use crate::model::vo::TaskVo;
use crate::model::MyResult;
use crate::service::task_service::TaskService;

pub(crate) async fn add(
    State(service): State<Arc<TaskService>>,
    Json(payload): Json<AddTaskParam>,
) -> Json<MyResult<i32>> {
    let id = service.add(payload);
    Json(MyResult::success(Some(id)))
}

pub(crate) async fn edit(
    State(service): State<Arc<TaskService>>,
    Json(payload): Json<EditTaskParam>,
) -> Json<MyResult<()>> {
    service.edit(&payload.into());
    Json(MyResult::success(None))
}

pub(crate) async fn delete(
    State(service): State<Arc<TaskService>>,
    Path(id): Path<i32>,
) -> Json<MyResult<()>> {
    service.delete(id);
    Json(MyResult::success(None))
}

pub(crate) async fn get(
    State(service): State<Arc<TaskService>>,
    Path(id): Path<i32>,
) -> Json<MyResult<TaskVo>> {
    let task = service.get(id);
    Json(MyResult::success(Some(task.into())))
}

pub(crate) async fn get_list(
    State(service): State<Arc<TaskService>>,
    Query(param): Query<TaskListParam>,
) -> Json<MyResult<Vec<TaskVo>>> {
    let tasks = service.get_list(&param);
    Json(MyResult::success(Some(tasks)))
}
