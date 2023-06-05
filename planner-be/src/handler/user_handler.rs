use axum::Json;

use crate::handler::jwt::new_token;
use crate::model::entity::User;
use crate::model::MyResult;
use crate::model::param::{AddUserParam, LoginParam};
use crate::repository::user_repository::UserRepository;

static USERNAME: &str = "admin";
static PASSWORD: &str = "fU0CtFrMOgYFFJVn";

pub(crate) struct UserHandler<'a> {
    repository: UserRepository<'a>,
}

impl<'a> UserHandler<'a> {
    pub(crate) fn new(repository: UserRepository<'a>) -> Self {
        Self {
            repository
        }
    }

    pub(crate) async fn login(&self, payload: LoginParam) -> Json<MyResult<String>> {
        let result = if payload.username == USERNAME && payload.password == PASSWORD {
            MyResult::success(Some(new_token()))
        } else {
            MyResult::fail(403, "username or password is wrong.")
        };
        Json(result)
    }

    pub(crate) async fn add(&mut self, payload: &AddUserParam) -> Json<MyResult<i32>> {
        let id = self.repository.insert(payload).unwrap();
        Json(MyResult::success(Some(id)))
    }

    pub(crate) async fn edit(&mut self, payload: &User) -> Json<MyResult<()>> {
        self.repository.update(payload).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn delete(&mut self, id: i32) -> Json<MyResult<()>> {
        self.repository.delete(id).unwrap();
        Json(MyResult::success(None))
    }

    pub(crate) async fn get(&mut self, id: i32) -> Json<MyResult<User>> {
        let user = self.repository.get(id).unwrap();
        Json(MyResult::success(Some(user)))
    }

    pub(crate) async fn get_all(&mut self) -> Json<MyResult<Vec<User>>> {
        let users = self.repository.get_all().unwrap();
        Json(MyResult::success(Some(users)))
    }
}
