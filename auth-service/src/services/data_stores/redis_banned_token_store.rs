use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::{BannedTokenStore, BannedTokenStoreError},
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
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let key = get_key(&token);
        
        // Get a write lock on the connection
        let mut conn = self.conn.write().await;
        
        // Cast TOKEN_TTL_SECONDS to u64
        let ttl = TOKEN_TTL_SECONDS as u64;
        
        // Set the key with expiration
        conn.set_ex::<_, _, ()>(key, true, ttl)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        
        Ok(())
    }

    async fn is_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let key = get_key(token);
        
        // Get a read lock on the connection
        let mut conn = self.conn.write().await;
        
        // Check if the key exists
        let exists: bool = conn.exists(&key)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        
        Ok(exists)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}