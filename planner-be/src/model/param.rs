use diesel::Insertable;
use serde::Deserialize;

use crate::schema::{sprint, task, user};

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

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = sprint)]
pub(crate) struct AddSprintParam {
    pub(crate) name: String,
    pub(crate) start: String,
    pub(crate) end: String,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = task)]
pub(crate) struct AddTaskParam {
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) sp: f64,
    pub(crate) tester: Option<i32>,
    pub(crate) test_sp: Option<f64>,
}
