use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub(crate) mod user_repository;
pub(crate) mod sprint_repository;
pub(crate) mod task_repository;

pub(crate) fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = "planner.db";

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
