use serde::Serialize;
use time::PrimitiveDateTime;

use crate::model::entity::Task;

#[derive(Serialize)]
pub(crate) struct TaskVo {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) sprint_name: Option<String>,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) developer_name: Option<String>,
    pub(crate) sp: String,
    pub(crate) tester: Option<i32>,
    pub(crate) tester_name: Option<String>,
    pub(crate) test_sp: Option<String>,
    pub(crate) start: Option<PrimitiveDateTime>,
    pub(crate) end: Option<PrimitiveDateTime>,
    pub(crate) test_start: Option<PrimitiveDateTime>,
    pub(crate) test_end: Option<PrimitiveDateTime>,
}

impl From<Task> for TaskVo {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sprint: value.sprint,
            sprint_name: None,
            ordinal: value.ordinal,
            developer: value.developer,
            developer_name: None,
            sp: value.sp.to_string(),
            tester: value.tester,
            tester_name: None,
            test_sp: value.test_sp.map(|e| e.to_string()),
            start: value.start,
            end: value.end,
            test_start: value.test_start,
            test_end: value.test_end,
        }
    }
}
