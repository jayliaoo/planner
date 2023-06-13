use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{
    insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

use crate::model::entity::User;
use crate::model::param::{AddUserParam, UserListParam};
use crate::{schema::user::dsl::user, schema::user::*};

pub(crate) struct UserRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl<'a> UserRepository {
    pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub(crate) fn insert(&self, param: &AddUserParam) -> QueryResult<i32> {
        self.pool.get().unwrap().transaction::<_, Error, _>(|conn| {
            insert_into(user).values(param).execute(conn)?;
            user.select(id).order(id.desc()).first(conn)
        })
    }

    pub(crate) fn update(&self, param: &User) -> QueryResult<usize> {
        diesel::update(param)
            .set(param)
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn delete(&self, id_: i32) -> QueryResult<usize> {
        diesel::delete(user)
            .filter(id.eq(id_))
            .execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get(&self, id_: i32) -> QueryResult<User> {
        user.filter(id.eq(id_))
            .get_result(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get_list(&self, param: &UserListParam) -> QueryResult<Vec<User>> {
        let mut query = table.into_boxed();
        if let Some(ids) = &param.ids {
            query = query.filter(id.eq_any(ids));
        }
        if let Some(role_) = &param.role {
            query = query.filter(role.eq(role_));
        }
        query.get_results(&mut self.pool.get().unwrap())
    }
}
