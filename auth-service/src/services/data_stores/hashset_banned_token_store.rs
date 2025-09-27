use crate::domain::{BannedTokenStore, BannedTokenStoreError};
use secrecy::{ExposeSecret, Secret};
use std::collections::HashSet;

pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

impl Default for HashsetBannedTokenStore {
    fn default() -> Self {
        Self {
            tokens: HashSet::new(),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.expose_secret().to_string());
        Ok(())
    }

    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = Secret::new("test_token_123".to_string());

        let result = store.add_token(token.clone()).await;
        assert!(result.is_ok());

        let is_banned = store.contains_token(&token).await.unwrap();
        assert!(is_banned);
    }

    #[tokio::test]
    async fn test_contains_token_returns_false_for_unknown_token() {
        let store = HashsetBannedTokenStore::default();
        let token = Secret::new("unknown_token".to_string());

        let is_banned = store.contains_token(&token).await.unwrap();
        assert!(!is_banned);
    }

    #[tokio::test]
    async fn test_add_multiple_tokens() {
        let mut store = HashsetBannedTokenStore::default();
        let token1 = Secret::new("token1".to_string());
        let token2 = Secret::new("token2".to_string());
        let token3 = Secret::new("token3".to_string());

        store.add_token(token1.clone()).await.unwrap();
        store.add_token(token2.clone()).await.unwrap();
        store.add_token(token3.clone()).await.unwrap();

        assert!(store.contains_token(&token1).await.unwrap());
        assert!(store.contains_token(&token2).await.unwrap());
        assert!(store.contains_token(&token3).await.unwrap());
        assert!(!store.contains_token(&Secret::new("unknown".to_string())).await.unwrap());
    }

    #[tokio::test]
    async fn test_add_duplicate_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = Secret::new("duplicate_token".to_string());

        store.add_token(token.clone()).await.unwrap();
        store.add_token(token.clone()).await.unwrap();

        // Should still be banned (no duplicates in HashSet)
        assert!(store.contains_token(&token).await.unwrap());
    }
}
