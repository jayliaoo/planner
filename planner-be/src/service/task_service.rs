use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::model::entity::Task;
use crate::model::param::{AddTaskParam, SprintListParam, TaskListParam, UserListParam};
use crate::model::vo::TaskVo;
use crate::repository::sprint_repository::SprintRepository;
use crate::repository::task_repository::TaskRepository;
use crate::repository::user_repository::UserRepository;

pub(crate) struct TaskService {
    task_repository: Arc<TaskRepository>,
    user_repository: Arc<UserRepository>,
    sprint_repository: Arc<SprintRepository>,
}

impl TaskService {
    pub(crate) fn new(
        task_repository: Arc<TaskRepository>,
        user_repository: Arc<UserRepository>,
        sprint_repository: Arc<SprintRepository>,
    ) -> Self {
        Self {
            task_repository,
            user_repository,
            sprint_repository,
        }
    }

    pub(crate) fn add(&self, payload: &mut AddTaskParam) -> i32 {
        let ordinal = self
            .task_repository
            .get_list(&TaskListParam {
                sprint: Some(payload.sprint),
                desc: Some(true),
                limit: Some(1),
            })
            .unwrap()
            .first()
            .map(|e| e.ordinal)
            .unwrap_or(0);
        payload.ordinal = ordinal + 1;
        self.task_repository.insert(payload).unwrap()
    }

    pub(crate) fn edit(&self, payload: &Task) -> usize {
        self.task_repository.update(payload).unwrap()
    }

    pub(crate) fn delete(&self, id: i32) -> usize {
        self.task_repository.delete(id).unwrap()
    }

    pub(crate) fn get(&self, id: i32) -> Task {
        self.task_repository.get(id).unwrap()
    }

    pub(crate) fn get_list(&self, param: &TaskListParam) -> Vec<TaskVo> {
        let tasks = self.task_repository.get_list(param).unwrap();

        let mut user_ids = HashSet::new();
        let mut sprint_ids = HashSet::new();
        for task in tasks.iter() {
            user_ids.insert(task.developer);
            if let Some(id) = task.tester {
                user_ids.insert(id);
            }
            sprint_ids.insert(task.sprint);
        }
        let users: HashMap<i32, String> = self
            .user_repository
            .get_list(&UserListParam {
                ids: Some(user_ids.into_iter().collect()),
                role: None,
            })
            .unwrap()
            .into_iter()
            .map(|e| (e.id, e.name))
            .collect();
        let sprints: HashMap<i32, String> = self
            .sprint_repository
            .get_list(&SprintListParam {
                ids: Some(sprint_ids.into_iter().collect()),
            })
            .unwrap()
            .into_iter()
            .map(|e| (e.id, e.name))
            .collect();

        tasks
            .into_iter()
            .map(|e| TaskVo {
                id: e.id,
                name: e.name,
                sprint: e.sprint,
                sprint_name: sprints.get(&e.sprint).unwrap().to_string(),
                ordinal: e.ordinal,
                developer: e.developer,
                developer_name: users
                    .get(&e.developer)
                    .map_or("".to_string(), |e| e.to_string()),
                sp: e.sp,
                tester: e.tester,
                tester_name: e
                    .tester
                    .map_or(None, |e| users.get(&e).map(|e| e.to_string())),
                test_sp: e.test_sp,
                start: e.start,
                end: e.end,
                test_start: e.test_start,
                test_end: e.test_end,
            })
            .collect()
    }
}
