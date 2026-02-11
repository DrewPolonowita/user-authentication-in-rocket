use rocket::http::{Cookie, SameSite};
use rocket::time::Duration;
use crate::crypto::hash::PasswordHash;

pub fn create_refresh() -> Result<(Cookie<'static>, String), rustls::crypto::GetRandomFailed> {
    // Creating refresh token
    let mut bytes = [0u8; 32]; // 256 bits
    let provider = rustls::crypto::ring::default_provider();
    let rng = &provider.secure_random;

    let _ = rng.fill(&mut bytes)?;
    let refresh_token = hex::encode(bytes);
    let refresh_hash = PasswordHash::try_from(refresh_token.as_str()).expect("asd");

    let cookie = Cookie::build(("refresh_token", refresh_hash.value().clone()))
         .http_only(true)
         .secure(false)
         .same_site(SameSite::Lax)
         .path("/refresh")
         .max_age(Duration::days(30))
         .finish();

    Ok((cookie, refresh_hash.value()))
}