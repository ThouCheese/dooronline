use crate::models::user::User;
use crate::schema::user;
use argon2rs::verifier::Encoded;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use jwt::{decode, encode, Header, Validation};
use rand::rngs::OsRng;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};

// Hash a password with Argon2.
pub fn hash_password(password: &str) -> String {
    let salt = generate_salt().unwrap();
    let encoded_hash = Encoded::default2i(password.as_bytes(), &salt, &[], &[]).to_u8();
    String::from_utf8(encoded_hash).unwrap()
}

// Verifies a password with Argon2.
pub fn verify_password(encoded_hash: &str, plaintext_password: &str) -> Result<bool, String> {
    let encoded = Encoded::from_u8(encoded_hash.as_bytes()).or(Err("Could not read password"))?;
    Ok(encoded.verify(plaintext_password.as_bytes()))
}

// Generates a random salt for Argon2.
fn generate_salt() -> Result<[u8; 16], String> {
    let mut rng = OsRng::new().or(Err("Could not allocate randomizer"))?;
    let mut salt = [0u8; 16];
    rng.fill_bytes(&mut salt);
    Ok(salt)
}

pub fn parse_token(jwt: &str) -> Result<LoginTokenClaims, String> {
    let login_token_claims: LoginTokenClaims = decode(
        jwt,
        b"NEDXDtZpcMtl5wdCmvWz16nTPVpfCUQD",
        &Validation::default(),
    )
    .or(Err("Could not deserialize JWT"))?
    .claims;
    Ok(login_token_claims)
}

pub fn create_auth_token(user: &User) -> Result<String, String> {
    let claims = LoginTokenClaims { id: user.id, exp: now() + 3600 * 24 * 365 };
    let token = encode(
        &Header::default(),
        &claims,
        b"NEDXDtZpcMtl5wdCmvWz16nTPVpfCUQD",
    )
    .or(Err("Could not create JWT"))?;
    Ok(token)
}

// Struct with information used in the JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenClaims {
    id: i32,
    exp: u64,
}

pub fn validate_user_token(jwt: &str) -> Option<User> {
    parse_token(jwt).ok().and_then(|claims| {
        let conn = crate::db::sync_connection();
        user::table.find(claims.id).get_result::<User>(&conn).ok()
    })
    // .unwrap_or(None)
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

