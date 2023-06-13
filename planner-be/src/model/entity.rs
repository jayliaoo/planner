use std::str::FromStr;

use bigdecimal::BigDecimal;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime};

use crate::model::param::{AddTaskParam, EditTaskParam};
use crate::schema::{sprint, task, user};

#[derive(Queryable, Deserialize, Serialize, AsChangeset, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = user)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) role: String,
}

#[derive(Queryable, Deserialize, Serialize, AsChangeset, Identifiable)]
#[diesel(table_name = sprint)]
pub(crate) struct Sprint {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) start: Date,
    pub(crate) end: Date,
}

#[derive(Queryable, AsChangeset, Identifiable, Insertable)]
#[diesel(table_name = task)]
pub(crate) struct Task {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) sp: BigDecimal,
    pub(crate) tester: Option<i32>,
    pub(crate) test_sp: Option<BigDecimal>,
    pub(crate) start: Option<PrimitiveDateTime>,
    pub(crate) end: Option<PrimitiveDateTime>,
    pub(crate) test_start: Option<PrimitiveDateTime>,
    pub(crate) test_end: Option<PrimitiveDateTime>,
}

impl From<AddTaskParam> for Task {
    fn from(value: AddTaskParam) -> Self {
        Self {
            id: 0,
            name: value.name,
            sprint: value.sprint,
            ordinal: value.ordinal,
            developer: value.developer,
            sp: BigDecimal::from_str(&value.sp).unwrap(),
            tester: value.tester,
            test_sp: value.test_sp.map(|e| BigDecimal::from_str(&e).unwrap()),
            start: None,
            end: None,
            test_start: None,
            test_end: None,
        }
    }
}

impl From<EditTaskParam> for Task {
    fn from(value: EditTaskParam) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sprint: value.sprint,
            ordinal: value.ordinal,
            developer: value.developer,
            sp: BigDecimal::from_str(&value.sp).unwrap(),
            tester: value.tester,
            test_sp: value.test_sp.map(|e| BigDecimal::from_str(&e).unwrap()),
            start: value.start,
            end: value.end,
            test_start: value.test_start,
            test_end: value.test_end,
        }
    }
}
