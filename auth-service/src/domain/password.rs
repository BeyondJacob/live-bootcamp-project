use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password> {
        if s.len() >= 8 {
            Ok(Password(s))
        } else {
            Err(eyre!("Password must be at least 8 characters long"))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn valid_passwords_are_parsed_successfully() {
        let valid_passwords = vec![
            "password123",
            "very_long_password_with_many_characters",
            "12345678",
            "P@ssw0rd!",
        ];

        for password in valid_passwords {
            assert!(Password::parse(password.to_string()).is_ok());
        }
    }

    #[test]
    fn short_passwords_are_rejected() {
        let short_passwords = vec!["", "1", "1234567", "seven77"];

        for password in short_passwords {
            assert!(Password::parse(password.to_string()).is_err());
        }
    }

    #[test]
    fn exactly_8_chars_is_valid() {
        let password = "12345678";
        assert!(Password::parse(password.to_string()).is_ok());
    }

    #[quickcheck]
    fn valid_passwords_dont_panic(password: String) -> bool {
        let _ = Password::parse(password);
        true
    }

    #[test]
    fn password_as_ref_returns_inner_string() {
        let password_str = "password123";
        let password = Password::parse(password_str.to_string()).unwrap();
        assert_eq!(password.as_ref(), password_str);
    }
}
