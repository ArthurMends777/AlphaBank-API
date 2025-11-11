use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

use crate::models::Claims;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "86400".to_string())
        .parse::<usize>()
        .unwrap_or(86400);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now().timestamp() as usize) + expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn validate_cpf(cpf: &str) -> bool {
    let cpf: String = cpf.chars().filter(|c| c.is_numeric()).collect();

    if cpf.len() != 11 {
        return false;
    }

    if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
        return false;
    }

    let mut sum = 0;
    for (i, c) in cpf[0..9].chars().enumerate() {
        sum += c.to_digit(10).unwrap() as usize * (10 - i);
    }
    let first_digit = match sum % 11 {
        0 | 1 => 0,
        n => 11 - n,
    };

    if first_digit != cpf.chars().nth(9).unwrap().to_digit(10).unwrap() as usize {
        return false;
    }

    sum = 0;
    for (i, c) in cpf[0..10].chars().enumerate() {
        sum += c.to_digit(10).unwrap() as usize * (11 - i);
    }
    let second_digit = match sum % 11 {
        0 | 1 => 0,
        n => 11 - n,
    };

    second_digit == cpf.chars().nth(10).unwrap().to_digit(10).unwrap() as usize
}
