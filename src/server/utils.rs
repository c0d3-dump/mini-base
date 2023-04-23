use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::model::{TokenUser, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user: TokenUser,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp_in = Duration::seconds(60);
    now += exp_in;
    let exp = now.timestamp() as usize;

    let token_user = TokenUser {
        id: user.id,
        email: user.email,
        role: user.role,
    };

    let claim = TokenClaims {
        exp,
        iat,
        user: token_user,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn decode_token(token: &str) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
}

pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hash = hasher.finalize();

    format!("{:x}", hash)
}
