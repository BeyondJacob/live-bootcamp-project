use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if validator::validate_email(&s) {
            Ok(Email(s))
        } else {
            Err(format!("{} is not a valid email address", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck_macros::quickcheck;

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let valid_emails = vec![
            "user@example.com",
            "test.email@domain.co.uk",
            "firstname.lastname@company.org",
            "email+tag@example.com",
            "user123@test-domain.com",
        ];

        for email in valid_emails {
            assert!(Email::parse(email.to_string()).is_ok());
        }
    }

    #[test]
    fn invalid_emails_are_rejected() {
        let invalid_emails = vec![
            "",
            "not-an-email",
            "@example.com",
            "user@",
            "user @example.com",
        ];

        for email in invalid_emails {
            assert!(
                Email::parse(email.to_string()).is_err(),
                "Expected '{}' to be invalid, but it was accepted",
                email
            );
        }
    }

    #[quickcheck]
    fn valid_emails_dont_panic(email: String) -> bool {
        let _ = Email::parse(email);
        true
    }

    #[test]
    fn fake_emails_are_valid() {
        for _ in 0..100 {
            let email: String = SafeEmail().fake();
            assert!(Email::parse(email).is_ok());
        }
    }

    #[test]
    fn email_as_ref_returns_inner_string() {
        let email_str = "test@example.com";
        let email = Email::parse(email_str.to_string()).unwrap();
        assert_eq!(email.as_ref(), email_str);
    }
}