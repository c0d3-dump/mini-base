use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token() -> Result<String, jsonwebtoken::errors::Error> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_in = Duration::seconds(60);
    now += exp_in;
    let exp = now.timestamp() as usize;

    let claim = TokenClaims { exp, iat };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn verify_token(token: &str) -> bool {
    let token = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    match token {
        Ok(_) => true,
        Err(_) => false,
    }
}
