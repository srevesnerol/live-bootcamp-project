use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        let key = get_key(token.as_str());
        // 2. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should simply be a `true` (boolean value).
        // The expiration time should be set to TOKEN_TTL_SECONDS.
        // NOTE: The TTL is expected to be a u64 so you will have to cast TOKEN_TTL_SECONDS to a u64.
        // Return BannedTokenStoreError::UnexpectedError if casting fails or the call to set_ex fails.

        let ttl = TOKEN_TTL_SECONDS as u64;
        match self.conn.write().await.set_ex::<std::string::String, bool, u64>(key, true, ttl) {
            Ok(_) => Ok(()),
            Err(_) => Err(BannedTokenError::UnexpectedError),
        }
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenError> {
            // Check if the token exists by calling the exists method on the Redis connection
            let key = get_key(token);
            match self.conn.write().await.exists(key) {
                Ok(value) => Ok(value),
                Err(_) => Err(BannedTokenError::UnexpectedError),
            }
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
