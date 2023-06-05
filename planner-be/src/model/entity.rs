use diesel::{AsChangeset, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::{sprint, task, user};

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = user)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) role: String,
}

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = sprint)]
pub(crate) struct Sprint {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) start: String,
    pub(crate) end: String,
}

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = task)]
pub(crate) struct Task {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) sp: f64,
    pub(crate) tester: Option<i32>,
    pub(crate) test_sp: Option<f64>,
    pub(crate) start: Option<String>,
    pub(crate) end: Option<String>,
    pub(crate) test_start: Option<String>,
    pub(crate) test_end: Option<String>,
}
