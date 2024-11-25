use std::sync::Arc;
use axum::{
    extract::Request, middleware::Next, response::Response, Extension
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;


use crate::AppState;

#[derive(Debug, Deserialize)]
struct CertResponse {
    public_cert: PublicCert,
}

#[derive(Debug, Deserialize)]
struct PublicCert {
    cert: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudflareJWTClaims {
    #[serde(rename = "sub")]
    pub sub: String,
    #[serde(rename = "email")]
    pub email: String,
    pub name: String,
    #[serde(rename = "iat")]
    pub issued_at: i64,
    #[serde(rename = "exp")]
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct CloudflareConfig {
    team_name: String,
    aud: String,
    pub_key_url: String,
}

impl CloudflareConfig {
    pub fn new(team_name: String, aud: String) -> Self {
        let pub_key_url = format!("https://{}.cloudflareaccess.com/cdn-cgi/access/certs", team_name);
        Self {
            team_name,
            aud,
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
        let client = reqwest::Client::new();
        let resp = client.get(&config.pub_key_url).send().await?;
        let cert_response: CertResponse = resp.json().await?;
        
        Ok(Self {
            config: Arc::new(config),
            public_key: Arc::new(cert_response.public_cert.cert),
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
    state: Extension<Arc<AppState>>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (axum::http::StatusCode, String)> {
    // Extract and validate the token
    let token = auth.token();
    let claims = state.cf_auth.validate_token(token).await
        .map_err(|e| (axum::http::StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e)))?;
    
    // Get or create user
    let user = state.get_or_create_user(&claims).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    // Check if user is active
    if !user.is_active {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "User account is not active".to_string(),
        ));
    }
    
    // Add user and claims to request extensions
    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(user);
    
    Ok(next.run(request).await)
}