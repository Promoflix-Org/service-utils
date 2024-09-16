use anyhow::Error;
use axum::extract::TypedHeader;
use chrono::Duration;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::ops::Add;
use std::str::FromStr;
use uuid::Uuid;

// use crate::server::grpc::check_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Default, Debug, Serialize, Clone, JsonSchema, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

fn gen_jwt(claims: &Claims) -> String {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("promoflix".as_ref()),
    );
    token.map_or("".to_string(), |f| f)
}

pub fn create_token(user_id: &Uuid, email: &String, role: &String) -> Token {
    let acc_claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_string(),
        role: role.to_string(),
        exp: chrono::Utc::now().add(Duration::days(7)).timestamp() as usize,
    };

    let ref_claims = Claims {
        sub: user_id.to_owned(),
        email: "".to_string(),
        role: "".to_string(),
        exp: chrono::Utc::now().add(Duration::days(30)).timestamp() as usize,
    };

    let access_token = gen_jwt(&acc_claims);
    let refresh_token = gen_jwt(&ref_claims);
    Token {
        access_token,
        refresh_token,
    }
}

pub async fn jwt_auth(
    TypedHeader(cookies): TypedHeader<Authorization<Bearer>>,
) -> Result<(Uuid, String, String), Error> {
    let token = cookies.0.token();
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(b"promoflix"), &validation)
        .map_err(|e| match *e.kind() {
            ErrorKind::InvalidToken => anyhow::anyhow!("Token is invalid"),
            ErrorKind::InvalidIssuer => anyhow::anyhow!("Issuer is invalid"),
            ErrorKind::ExpiredSignature => anyhow::anyhow!("Token expired"),
            _ => anyhow::anyhow!("Some other errors"),
        })?;

    let user_id = &token_data.claims.sub;
    let email = &token_data.claims.email;
    let role = &token_data.claims.role;
    if email.is_empty() {
        return Err(Error::msg("Email is empty".to_string()));
    }
    if role.is_empty() {
        return Err(Error::msg("Role is empty".to_string()));
    }

    Ok((user_id.to_owned(), email.to_string(), role.to_string()))
}

pub async fn jwt_str_auth(token: &String) -> Result<(Uuid, String, String), Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(b"promoflix"), &validation)
        .map_err(|e| match *e.kind() {
            ErrorKind::InvalidToken => anyhow::anyhow!("Token is invalid"),
            ErrorKind::InvalidIssuer => anyhow::anyhow!("Issuer is invalid"),
            ErrorKind::ExpiredSignature => anyhow::anyhow!("Token expired"),
            _ => anyhow::anyhow!("Some other errors"),
        })?;

    let user_id = &token_data.claims.sub;
    let email = &token_data.claims.email;
    let role = &token_data.claims.role;
    if email.is_empty() {
        return Err(Error::msg("Email is empty".to_string()));
    }
    if role.is_empty() {
        return Err(Error::msg("Role is empty".to_string()));
    }

    Ok((user_id.to_owned(), email.to_string(), role.to_string()))
}

#[derive(Debug, Clone, JsonSchema, PartialEq, Serialize, Deserialize)]
pub enum TokenRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "guest")]
    Guest,
    #[serde(rename = "user")]
    DefaultTokenRole,
}

impl Default for TokenRole {
    fn default() -> Self {
        Self::DefaultTokenRole
    }
}

impl fmt::Display for TokenRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TokenRole {
    type Err = ();
    fn from_str(input: &str) -> Result<TokenRole, Self::Err> {
        match input {
            "Admin" => Ok(TokenRole::Admin),
            "User" => Ok(TokenRole::User),
            "Guest" => Ok(TokenRole::Guest),
            _ => Err(()),
        }
    }
}
