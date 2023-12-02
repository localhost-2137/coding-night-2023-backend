use axum_extra::extract::CookieJar;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct JWTAuth {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub id: u32,
}

pub fn serialize_jwt(val: JWTAuth) -> anyhow::Result<String> {
    let secret = std::env::var("SECRET")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let header = get_jwt_header();
    let token_str = Token::new(header, val).sign_with_key(&key)?;

    Ok(token_str.as_str().to_string())
}

pub fn extract_jwt(cookie_jar: &CookieJar) -> anyhow::Result<JWTAuth> {
    let secret = std::env::var("SECRET")?;

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let token_str = token_from_headers(cookie_jar)?;

    let token: Token<Header, JWTAuth, _> =
        VerifyWithKey::verify_with_key(token_str.as_str(), &key)?;

    Ok(token.claims().to_owned())
}

pub fn token_from_headers(cookie_jar: &CookieJar) -> anyhow::Result<String> {
    let cookie = cookie_jar
        .get("JWT_AUTH")
        .ok_or(anyhow::Error::msg("Failed to read AUTH_JWT header"))?;
    let token = cookie.value().to_string();

    Ok(token)
}

fn get_jwt_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    }
}
