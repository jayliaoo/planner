use diesel::{Connection, ExpressionMethods, insert_into, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;

use crate::{schema::user::*, schema::user::dsl::user};
use crate::model::entity::User;
use crate::model::param::AddUserParam;

pub(crate) struct UserRepository<'a> {
    pool: &'a Pool<ConnectionManager<SqliteConnection>>,
}

impl<'a> UserRepository<'a> {
    pub(crate) fn new(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self {
            pool
        }
    }

    pub(crate) fn insert(&self, param: &AddUserParam) -> QueryResult<i32> {
        self.pool.get().unwrap().transaction::<_, Error, _>(|conn| {
            insert_into(user).values(name.eq(&param.name)).execute(conn)?;
            user.select(id).order(id.desc()).first(conn)
        })
    }

    pub(crate) fn update(&self, param: &User) -> QueryResult<usize> {
        diesel::update(user).filter(id.eq(param.id)).set(name.eq(&param.name)).execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn delete(&self, id_: i32) -> QueryResult<usize> {
        diesel::delete(user).filter(id.eq(id_)).execute(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get(&self, id_: i32) -> QueryResult<User> {
        user.filter(id.eq(id_)).get_result(&mut self.pool.get().unwrap())
    }

    pub(crate) fn get_all(&self) -> QueryResult<Vec<User>> {
        user.get_results(&mut self.pool.get().unwrap())
    }
}

