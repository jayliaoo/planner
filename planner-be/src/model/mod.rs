use serde::Deserialize;
use serde::Serialize;

pub(crate) mod param;
pub(crate) mod entity;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) exp: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PageResult<T> {
    pub(crate) total_count: i64,
    pub(crate) list: Vec<T>,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MyResult<T> {
    pub(crate) result_code: i32,
    pub(crate) message: String,
    pub(crate) data: Option<T>,
}

impl<T> MyResult<T> {
    pub(crate) fn success(data: Option<T>) -> Self {
        MyResult {
            result_code: 200,
            message: "SUCCESS".to_string(),
            data,
        }
    }

    pub(crate) fn fail(code: i32, message: &str) -> Self {
        MyResult {
            result_code: code,
            message: message.to_string(),
            data: None,
        }
    }
}
