use axum::Json;

use crate::model::entity::Sprint;
use crate::model::MyResult;
use crate::model::param::AddSprintParam;
use crate::repository::sprint_repository::SprintRepository;

pub(crate) struct SprintHandler<'a> {
    repository: SprintRepository<'a>,
}

impl<'a> SprintHandler<'a> {
    pub(crate) fn new(repository: SprintRepository) -> Self {
        Self {
            repository
        }
    }

    pub(crate) async fn add(&mut self, payload: &AddSprintParam) -> Json<MyResult<i32>> {
        let id = self.repository.insert(payload).unwrap();
        Json(MyResult::success(Some(id)))
    }

    pub(crate) async fn edit(&mut self, payload: &Sprint) -> Json<MyResult<()>> {
        self.repository.update(payload).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn delete(&mut self, id: i32) -> Json<MyResult<()>> {
        self.repository.delete(id).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn get(&mut self, id: i32) -> Json<MyResult<Sprint>> {
        let sprint = self.repository.get(id).unwrap();
        Json(MyResult::success(Some(sprint)))
    }

    pub(crate) async fn get_all(&mut self) -> Json<MyResult<Vec<Sprint>>> {
        let sprints = self.repository.get_all().unwrap();
        Json(MyResult::success(Some(sprints)))
    }
}
