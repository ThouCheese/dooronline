use argon2rs::verifier::Encoded;
use jwt::{decode, encode, Header, Validation};
use models::User;
use schema::user;
#[cfg(target_arch = "x86_64")]
use rand::{OsRng, Rng};
#[cfg(target_arch = "arm")]
use rand::{OsRng, RngCore};
use db::get_connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

// Hash a password with Argon2.
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = generate_salt()?;
    let encoded_hash = Encoded::default2i(password.as_bytes(), &salt, &[], &[]).to_u8();
    Ok(String::from_utf8(encoded_hash)
        .or( Err("Something went wrong creating your password"))?
    )
}

// Verifies a password with Argon2.
pub fn verify_password(encoded_hash: &str, plaintext_password: &str) -> Result<bool, String> {
    let encoded = Encoded::from_u8(encoded_hash.as_bytes())
        .or(Err("Could not read password"))?;
    Ok(encoded.verify(plaintext_password.as_bytes()))
}

// Generates a random salt for Argon2.
fn generate_salt() -> Result<[u8; 16], String> {
    let mut rng = OsRng::new()
        .or(Err("Could not allocate randomizer"))?;
    let mut salt = [0u8; 16];
    rng.fill_bytes(&mut salt);
    Ok(salt)
}

pub fn parse_token(jwt: &str) -> Result<LoginTokenClaims, String> {
    let login_token_claims: LoginTokenClaims = decode(
        jwt, b"NEDXDtZpcMtl5wdCmvWz16nTPVpfCUQD", &Validation::default()
    ).or(Err("Could not deserialize JWT"))?.claims;
    Ok(login_token_claims)
}

pub fn create_auth_token(user: &User) -> Result<String, String> {
    let claims = LoginTokenClaims {
        id: user.id,
    };
    let token = encode(&Header::default(), &claims, b"NEDXDtZpcMtl5wdCmvWz16nTPVpfCUQD")
        .or(Err("Could not create JWT")
    )?;
    Ok(token)
}

// Struct with information used in the JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenClaims {
    pub id: i32,
}

pub fn validate_user_token(jwt: &str) -> Option<User> {
    parse_token(jwt)
        .ok()
        .map(
            |claims| user::table.find(claims.id)
                .get_result::<User>(&get_connection()).ok()
        )
        .unwrap_or(None)
}
