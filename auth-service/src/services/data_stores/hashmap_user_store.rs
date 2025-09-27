use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if &user.password == password {
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
    use secrecy::Secret;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
            password: Password::parse(Secret::new("password123".to_string())).unwrap(),
            requires_2fa: false,
        };

        // Test successful addition
        assert!(store.add_user(user.clone()).await.is_ok());

        // Test duplicate user error
        assert_eq!(
            store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
            password: Password::parse(Secret::new("password123".to_string())).unwrap(),
            requires_2fa: true,
        };

        // Test user not found
        assert_eq!(
            store
                .get_user(&Email::parse(Secret::new("test@example.com".to_string())).unwrap())
                .await,
            Err(UserStoreError::UserNotFound)
        );

        // Add user and test successful retrieval
        store.add_user(user.clone()).await.unwrap();
        assert_eq!(
            store
                .get_user(&Email::parse(Secret::new("test@example.com".to_string())).unwrap())
                .await,
            Ok(user)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
            password: Password::parse(Secret::new("password123".to_string())).unwrap(),
            requires_2fa: false,
        };

        // Test user not found
        assert_eq!(
            store
                .validate_user(
                    &Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
                    &Password::parse(Secret::new("password123".to_string())).unwrap()
                )
                .await,
            Err(UserStoreError::UserNotFound)
        );

        // Add user
        store.add_user(user).await.unwrap();

        // Test successful validation
        assert!(store
            .validate_user(
                &Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
                &Password::parse(Secret::new("password123".to_string())).unwrap()
            )
            .await
            .is_ok());

        // Test invalid credentials
        assert_eq!(
            store
                .validate_user(
                    &Email::parse(Secret::new("test@example.com".to_string())).unwrap(),
                    &Password::parse(Secret::new("wrongpassword".to_string())).unwrap()
                )
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
