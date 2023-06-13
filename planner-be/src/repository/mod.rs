use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub(crate) mod sprint_repository;
pub(crate) mod task_repository;
pub(crate) mod user_repository;

pub(crate) fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = "postgres://jayliao:password@localhost/planner";

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
