use diesel::Insertable;
use serde::Deserialize;
use time::{Date, PrimitiveDateTime};

use crate::schema::{sprint, user};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginParam {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = user)]
pub(crate) struct AddUserParam {
    pub(crate) name: String,
    pub(crate) role: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserListParam {
    pub(crate) ids: Option<Vec<i32>>,
    pub(crate) role: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SprintListParam {
    pub(crate) ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TaskListParam {
    pub(crate) sprint: Option<i32>,
    pub(crate) desc: Option<bool>,
    pub(crate) limit: Option<i64>,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = sprint)]
pub(crate) struct AddSprintParam {
    pub(crate) name: String,
    pub(crate) start: Date,
    pub(crate) end: Date,
}

#[derive(Deserialize, Debug)]
pub(crate) struct AddTaskParam {
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) sp: String,
    pub(crate) tester: Option<i32>,
    pub(crate) test_sp: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct EditTaskParam {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) sp: String,
    pub(crate) tester: Option<i32>,
    pub(crate) test_sp: Option<String>,
    pub(crate) start: Option<PrimitiveDateTime>,
    pub(crate) end: Option<PrimitiveDateTime>,
    pub(crate) test_start: Option<PrimitiveDateTime>,
    pub(crate) test_end: Option<PrimitiveDateTime>,
}
