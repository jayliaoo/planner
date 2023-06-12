use std::marker::PhantomData;

use axum::http::HeaderValue;
use http_body::Body;
use hyper::{header, Request, Response, StatusCode};
use jwt_simple::prelude::*;
use jwt_simple::Error;
use tower_http::validate_request::ValidateRequest;

pub(crate) struct BaseJwt {
    key: HS256Key,
}

impl BaseJwt {
    pub(crate) fn new(secret: String) -> Self {
        Self {
            key: HS256Key::from_bytes(secret.as_bytes()),
        }
    }

    fn verify(&self, header: &HeaderValue) -> Result<JWTClaims<NoCustomClaims>, Error> {
        match header.to_str().unwrap().strip_prefix("Bearer ") {
            Some(token) => self.key.verify_token::<NoCustomClaims>(&token, None),
            None => Err(Error::msg("The token is not present.")),
        }
    }

    pub(crate) fn new_token(&self) -> String {
        let claims = Claims::create(Duration::from_days(1));
        self.key.authenticate(claims).unwrap()
    }
}

impl Clone for BaseJwt {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
        }
    }
}

pub(crate) struct Jwt<ResBody> {
    jwt: BaseJwt,
    _pd: PhantomData<ResBody>,
}

impl<ResBody> Jwt<ResBody> {
    pub(crate) fn new(jwt: BaseJwt) -> Self {
        Self {
            jwt,
            _pd: PhantomData,
        }
    }
}

impl<ResBody> Clone for Jwt<ResBody> {
    fn clone(&self) -> Self {
        Self {
            jwt: self.jwt.clone(),
            _pd: PhantomData,
        }
    }
}

impl<B, ResBody: Body + Default> ValidateRequest<B> for Jwt<ResBody> {
    type ResponseBody = ResBody;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        match request.headers().get(header::AUTHORIZATION) {
            Some(actual) => match self.jwt.verify(actual) {
                Ok(_) => Ok(()),
                _ => {
                    let mut res = Response::new(ResBody::default());
                    *res.status_mut() = StatusCode::UNAUTHORIZED;
                    Err(res)
                }
            },
            _ => {
                let mut res = Response::new(ResBody::default());
                *res.status_mut() = StatusCode::UNAUTHORIZED;
                Err(res)
            }
        }
    }
}
