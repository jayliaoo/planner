use axum::http::HeaderValue;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use time::{Duration, OffsetDateTime};

use crate::model::Claims;

static SECRET: &str = "6d4f5439-e33a-45e1-b914-9bbe18ee7137";

pub(crate) fn decode_token(header: &HeaderValue) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let token = header.to_str().unwrap();
    let key = DecodingKey::from_secret(SECRET.as_ref());
    decode::<Claims>(token, &key, &Validation::default())
}

pub(crate) fn new_token() -> String {
    let exp = ((OffsetDateTime::now_utc() + Duration::days(1)).unix_timestamp()) as usize;
    let key = EncodingKey::from_secret(SECRET.as_ref());
    encode(&Header::default(), &Claims { exp }, &key).unwrap()
}
