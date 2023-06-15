use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub(crate) mod sprint_repository;
pub(crate) mod task_repository;
pub(crate) mod user_repository;

pub(crate) fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = "postgres://citus:U8prCDk!ks2cr4p@c-default.wt2pbdaaozel2d.postgres.cosmos.azure.com:5432/default?sslmode=require";

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
