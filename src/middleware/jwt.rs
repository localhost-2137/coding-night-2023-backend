use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use crate::utils::jwt::JWTAuth;

#[async_trait]
impl<S> FromRequestParts<S> for JWTAuth
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_headers(&parts.headers);
        
        crate::utils::jwt::extract_jwt(&cookie_jar).map_err(|_| (StatusCode::UNAUTHORIZED, "Failed to authorize"))
    }
}
