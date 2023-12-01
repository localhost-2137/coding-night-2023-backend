use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use hmac::digest::KeyInit;
use hmac::{Hmac};
use jwt::{AlgorithmType, Header, VerifyWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
struct ExtractJWT {
    email: String,
    id: u32,
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractJWT
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        extract_jwt(parts).or_else(|_| {
            Err((StatusCode::UNAUTHORIZED, "Failed to authorize"))
        })
    }
}

fn extract_jwt(parts: &mut Parts) -> anyhow::Result<ExtractJWT> {
    let secret = std::env::var("SECRET")?;
    
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    let token_str = token_from_headers(&parts.headers)?;
    let _header = Header{
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };
    
    let token: Token<Header, ExtractJWT, _> = VerifyWithKey::verify_with_key(token_str.as_str(), &key)?;

    Ok(token.claims().to_owned())
}

fn token_from_headers(headers: &HeaderMap<HeaderValue>) -> anyhow::Result<String> {
    let token = headers.get("AUTH_JWT").ok_or(
        anyhow::Error::msg("Failed to read AUTH_JWT header")
    )?;
    let token = token.to_str()?.to_string();

    Ok(token)
}
