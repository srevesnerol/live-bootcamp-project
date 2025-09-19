use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug)]
struct UserRow {
    email: String,
    password_hash: String,
    requires_2fa: bool,
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    // TODO: Implement all required methods. Note that you will need to make SQL queries against our PostgreSQL instance inside these methods.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let exists = sqlx::query_scalar!(
            r#"select exists(select 1 from users where email = $1) as "exists!""#,
            user.email.as_ref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        if exists {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            let password_hash = compute_password_hash(user.password.as_ref())
                .await
                .map_err(|_| UserStoreError::UnexpectedError)?;

            sqlx::query!(
                "insert into users (email, password_hash, requires_2fa) values ($1, $2, $3)",
                user.email.as_ref(),
                password_hash.to_string(),
                user.requires_2fa
            )
            .execute(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user = sqlx::query_as!(
            UserRow,
            "select email, password_hash, requires_2fa from users where email = $1",
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        match user {
            Some(user) => Ok(User {
                email: Email::parse(user.email).unwrap(),
                password: Password::parse(user.password_hash).unwrap(),
                requires_2fa: user.requires_2fa,
            }),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {

        let user = sqlx::query_as!(
            UserRow,
            "select email, password_hash, requires_2fa from users where email = $1",
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        match user {
            Some(user) => {
                if verify_password_hash(&user.password_hash, password.as_ref()).await.is_ok() {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

async fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(password_hash)
}

async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash: PasswordHash<'_> = PasswordHash::new(expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}
