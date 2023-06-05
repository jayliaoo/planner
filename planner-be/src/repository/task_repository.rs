use diesel::{Connection, ExpressionMethods, insert_into, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;

use crate::{schema::task::*, schema::task::dsl::task};
use crate::model::entity::Task;
use crate::model::param::AddTaskParam;

pub(crate) struct TaskRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl TaskRepository {
    pub(crate) fn new(pool:  Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self {
            pool
        }
    }

    pub(crate) fn insert(&self, param: &AddTaskParam) -> QueryResult<i32> {
        self.pool.get().unwrap().transaction::<_, Error, _>(|conn| {
            insert_into(task).values(param).execute(conn)?;
            task.select(id).order(id.desc()).first(conn)
        })
    }

    pub(crate) fn update(&self, param: &Task) -> QueryResult<usize> {
        diesel::update(task).set(param).execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn delete(&self, id_: i32) -> QueryResult<usize> {
        diesel::delete(task).filter(id.eq(id_)).execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get(&self, id_: i32) -> QueryResult<Task> {
        task.filter(id.eq(id_)).get_result(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get_all(&self) -> QueryResult<Vec<Task>> {
        task.get_results(&mut self.pool.get().unwrap())
    }
}
