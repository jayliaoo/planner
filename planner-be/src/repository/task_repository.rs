use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{
    insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

use crate::model::entity::Task;
use crate::model::param::TaskListParam;
use crate::{schema::task::dsl::task, schema::task::*};

pub(crate) struct TaskRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl TaskRepository {
    pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub(crate) fn insert(&self, param: &Task) -> QueryResult<i32> {
        self.pool.get().unwrap().transaction::<_, Error, _>(|conn| {
            insert_into(task).values(param).execute(conn)?;
            task.select(id).order_by(id.desc()).first(conn)
        })
    }

    pub(crate) fn update(&self, param: &Task) -> QueryResult<usize> {
        diesel::update(param)
            .set(param)
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn delete(&self, id_: i32) -> QueryResult<usize> {
        diesel::delete(task)
            .filter(id.eq(id_))
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get(&self, id_: i32) -> QueryResult<Task> {
        task.filter(id.eq(id_))
            .get_result(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get_list(&self, param: &TaskListParam) -> QueryResult<Vec<Task>> {
        let mut query = table.into_boxed();
        if let Some(sprint_id) = param.sprint {
            query = query.filter(sprint.eq(sprint_id));
        }
        if let Some(true) = param.desc {
            query = query.order_by(ordinal.desc())
        } else {
            query = query.order_by(ordinal)
        }
        if let Some(limit) = param.limit {
            query = query.limit(limit);
        }
        query.get_results(&mut self.pool.get().unwrap())
    }
}
