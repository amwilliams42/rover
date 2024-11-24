use std::{sync::Arc, time::{Duration, SystemTime}};
use axum::{
    extract::{Request, State}, middleware::Next, response::{IntoResponse, Response}
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use hyper::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use reqwest::Client;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::AppState;

#[derive(Deserialize)]
struct CloudflareCertResponse {
    public_cert: CloudFlareCert,
}

#[derive(Deserialize)]
struct CloudFlareCert {
    cert: String,
}

static CERT_CACHE: Lazy<RwLock<Option<(String, SystemTime)>>> = Lazy::new(|| RwLock::new(None));
const CERT_REFRESH_SECONDS: u64 = 60 * 60 * 24;

#[derive(Debug, Clone, Deserialize)]
pub struct CloudflareJWTClaims {
    #[serde(rename = "sub")]
    pub sub: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "iat")]
    pub issued_at: i64,
    #[serde(rename = "exp")]
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct CloudflareConfig {
    team_domain: String,
    aud: String,
    pub_key_url: String,
}

impl CloudflareConfig {
    pub fn new(team_domain: &str, aud: &str) -> Self {
        let pub_key_url = format!("https://{}/cdn-cgi/access/certs", team_domain);
        Self {
            team_domain: team_domain.to_string(),
            aud: aud.to_string(),
            pub_key_url,
        }
    }
}

#[derive(Clone)]
pub struct CloudflareAuth {
    config: Arc<CloudflareConfig>,
    public_key: Arc<String>,
}

impl CloudflareAuth {
    pub async fn new(config: CloudflareConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Fetch public key from Cloudflare
        let client = reqwest::Client::new();
        let resp = client.get(&config.pub_key_url).send().await?;
        let keys: serde_json::Value = resp.json().await?;
        
        // Extract the public key (you might need to adjust this based on the exact response format)
        let public_key = keys["public_cert"]
            .as_str()
            .ok_or("Failed to get public key")?
            .to_string();

        Ok(Self {
            config: Arc::new(config),
            public_key: Arc::new(public_key),
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<CloudflareJWTClaims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        let key = DecodingKey::from_rsa_pem(self.public_key.as_bytes())?;
        
        let token_data = decode::<CloudflareJWTClaims>(
            token,
            &key,
            &validation,
        )?;
        
        Ok(token_data.claims)
    }
}

pub async fn cloudflare_auth_middleware(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    cf_auth: Arc<CloudflareAuth>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (axum::http::StatusCode, String)> {
    // Extract the token
    let token = auth.token();
    
    // Validate the token
    match cf_auth.validate_token(token).await {
        Ok(claims) => {
            // Add claims to request extensions for later use
            request.extensions_mut().insert(claims);
            
            // Continue with the request
            Ok(next.run(request).await)
        }
        Err(e) => {
            Err((
                axum::http::StatusCode::UNAUTHORIZED,
                format!("Invalid token: {}", e),
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    CadUser,
    CadManager,
    CadAdmin,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub roles: Vec<Role>,
}