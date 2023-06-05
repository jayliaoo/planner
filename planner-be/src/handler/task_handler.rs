use axum::extract::Path;
use axum::Json;

use crate::model::entity::Task;
use crate::model::MyResult;
use crate::model::param::AddTaskParam;
use crate::repository::task_repository::TaskRepository;

pub(crate) struct TaskHandler<'a> {
    repository: TaskRepository<'a>,
}

impl<'a> TaskHandler<'a> {
    pub(crate) fn new(repository: TaskRepository) -> Self {
        Self {
            repository
        }
    }

    pub(crate) async fn add(&mut self, payload: &AddTaskParam) -> Json<MyResult<i32>> {
        let id = self.repository.insert(payload).unwrap();
        Json(MyResult::success(Some(id)))
    }

    pub(crate) async fn edit(&mut self, payload: &Task) -> Json<MyResult<()>> {
        self.repository.update(payload).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn delete(&mut self, id: i32) -> Json<MyResult<()>> {
        self.repository.delete(id).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn get(&mut self, id: i32) -> Json<MyResult<Task>> {
        let task = self.repository.get(id).unwrap();
        Json(MyResult::success(Some(task)))
    }

    pub(crate) async fn get_all(&mut self) -> Json<MyResult<Vec<Task>>> {
        let tasks = self.repository.get_all().unwrap();
        Json(MyResult::success(Some(tasks)))
    }
}
