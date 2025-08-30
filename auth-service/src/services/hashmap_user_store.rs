use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };
        
        // Test successful addition
        assert!(store.add_user(user.clone()).is_ok());
        
        // Test duplicate user error
        assert_eq!(
            store.add_user(user),
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: true,
        };
        
        // Test user not found
        assert_eq!(
            store.get_user("test@example.com"),
            Err(UserStoreError::UserNotFound)
        );
        
        // Add user and test successful retrieval
        store.add_user(user.clone()).unwrap();
        assert_eq!(store.get_user("test@example.com"), Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };
        
        // Test user not found
        assert_eq!(
            store.validate_user("test@example.com", "password123"),
            Err(UserStoreError::UserNotFound)
        );
        
        // Add user
        store.add_user(user).unwrap();
        
        // Test successful validation
        assert!(store.validate_user("test@example.com", "password123").is_ok());
        
        // Test invalid credentials
        assert_eq!(
            store.validate_user("test@example.com", "wrongpassword"),
            Err(UserStoreError::InvalidCredentials)
        );
    }
}