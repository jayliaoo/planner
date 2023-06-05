use std::marker::PhantomData;

use axum::http::HeaderValue;
use http_body::Body;
use hyper::{header, Request, Response, StatusCode};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use time::{Duration, OffsetDateTime};
use tower_http::validate_request::ValidateRequest;

use crate::model::Claims;

pub(crate) struct BaseJwt {
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}

impl BaseJwt {
    pub(crate) fn new(secret: String) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
        }
    }

    fn verify(&self, header: &HeaderValue) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        let token = header.to_str().unwrap();
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
    }

    pub(crate) fn new_token(&self) -> String {
        let exp = ((OffsetDateTime::now_utc() + Duration::days(1)).unix_timestamp()) as usize;
        encode(&Header::default(), &Claims { exp }, &self.encoding_key).unwrap()
    }
}

impl Clone for BaseJwt {
    fn clone(&self) -> Self {
        Self {
            decoding_key: self.decoding_key.clone(),
            encoding_key: self.encoding_key.clone(),
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
            Some(actual) => {
                match self.jwt.verify(actual) {
                    Ok(_) => Ok(()),
                    _ => {
                        let mut res = Response::new(ResBody::default());
                        *res.status_mut() = StatusCode::UNAUTHORIZED;
                        Err(res)
                    },
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
