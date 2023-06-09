use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct TaskVo {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) sprint: i32,
    pub(crate) sprint_name: String,
    pub(crate) ordinal: i16,
    pub(crate) developer: i32,
    pub(crate) developer_name: String,
    pub(crate) sp: f64,
    pub(crate) tester: Option<i32>,
    pub(crate) tester_name: Option<String>,
    pub(crate) test_sp: Option<f64>,
    pub(crate) start: Option<String>,
    pub(crate) end: Option<String>,
    pub(crate) test_start: Option<String>,
    pub(crate) test_end: Option<String>,
}
