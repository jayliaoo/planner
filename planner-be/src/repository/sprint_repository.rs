use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{
    insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

use crate::model::entity::Sprint;
use crate::model::param::{AddSprintParam, SprintListParam};
use crate::{schema::sprint::dsl::sprint, schema::sprint::*};

pub(crate) struct SprintRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl SprintRepository {
    pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub(crate) fn insert(&self, param: &AddSprintParam) -> QueryResult<i32> {
        self.pool.get().unwrap().transaction::<_, Error, _>(|conn| {
            insert_into(sprint).values(param).execute(conn)?;
            sprint.select(id).order(id.desc()).first(conn)
        })
    }

    pub(crate) fn update(&self, param: &Sprint) -> QueryResult<usize> {
        diesel::update(param)
            .set(param)
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn delete(&self, id_: i32) -> QueryResult<usize> {
        diesel::delete(sprint)
            .filter(id.eq(id_))
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get(&self, id_: i32) -> QueryResult<Sprint> {
        sprint
            .filter(id.eq(id_))
            .get_result(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get_list(&self, param: &SprintListParam) -> QueryResult<Vec<Sprint>> {
        let mut query = table.into_boxed();
        if let Some(ids) = &param.ids {
            query = query.filter(id.eq_any(ids));
        }
        query
            .order_by(id.desc())
            .get_results(&mut self.pool.get().unwrap())
    }
}
