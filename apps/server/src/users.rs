use sqlx::{ types::Uuid, types::time::OffsetDateTime};
use serde::{Deserialize, Serialize};
use time::serde::timestamp;

use crate::AppState;
use crate::auth::CloudflareJWTClaims; // Import the CloudflareJwtClaims type

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    CadUser,
    CadManager,
    CadAdmin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(with = "uuid_serialization")]
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub roles: Vec<UserRole>,
    #[serde(with = "timestamp")]
    pub created_at: OffsetDateTime,
    #[serde(with = "timestamp")]
    pub last_login: OffsetDateTime,
    pub is_active: bool,
}

impl User {
    pub fn has_role(&self, role: &UserRole) -> bool {
        self.roles.contains(role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&UserRole::CadAdmin)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    #[serde(with = "uuid_serialization")]
    pub id: Uuid,
    #[serde(with = "uuid_serialization")]
    pub user_id: Uuid,
    #[serde(with = "timestamp")]
    pub last_ping: OffsetDateTime,
    #[serde(with = "timestamp")]
    pub created_at: OffsetDateTime,
}

// Custom UUID serialization module
mod uuid_serialization {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use sqlx::types::Uuid;

    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uuid.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s).map_err(serde::de::Error::custom)
    }
}

impl AppState {
    // Get or create user based on Cloudflare claims
    pub async fn get_or_create_user(&self, claims: &CloudflareJWTClaims) -> Result<User, sqlx::Error> {
        // Default roles for new users
        let default_roles = vec![UserRole::CadUser];
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, name, roles)
            VALUES ($1, $2, $3)
            ON CONFLICT (email) DO UPDATE
            SET last_login = NOW()
            RETURNING id, email, name, roles as "roles: Vec<UserRole>", created_at, last_login, is_active
            "#,
            claims.email,
            claims.name,
            &default_roles as &[UserRole],
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn update_user_roles(&self, user_id: Uuid, roles: &[UserRole]) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET roles = $1
            WHERE id = $2
            RETURNING id, email, name, roles as "roles: Vec<UserRole>", created_at, last_login, is_active
            "#,
            roles as &[UserRole],
            user_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn add_user_role(&self, user_id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET roles = array_append(roles, $1)
            WHERE id = $2 AND NOT $1 = ANY(roles)
            RETURNING id, email, name, roles as "roles: Vec<UserRole>", created_at, last_login, is_active
            "#,
            role as UserRole,
            user_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn remove_user_role(&self, user_id: Uuid, role: UserRole) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET roles = array_remove(roles, $1)
            WHERE id = $2
            RETURNING id, email, name, roles as "roles: Vec<UserRole>", created_at, last_login, is_active
            "#,
            role as UserRole,
            user_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<UserSession, sqlx::Error> {
        let session = sqlx::query_as!(
            UserSession,
            r#"
            INSERT INTO user_sessions (user_id)
            VALUES ($1)
            RETURNING id, user_id, last_ping, created_at
            "#,
            user_id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(session)
    }

    pub async fn update_session_ping(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE user_sessions
            SET last_ping = NOW()
            WHERE id = $1
            "#,
            session_id,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn delete_session(&self, session_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM user_sessions
            WHERE id = $1
            "#,
            session_id,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn cleanup_old_sessions(&self, max_age_minutes: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM user_sessions
            WHERE last_ping < NOW() - INTERVAL '1 minute' * $1
            "#,
            max_age_minutes as f64,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}