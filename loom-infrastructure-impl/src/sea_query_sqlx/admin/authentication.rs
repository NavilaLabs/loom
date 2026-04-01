pub mod jwt {
    use bcrypt::verify;
    use chrono::Utc;
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use loom_core::admin::authenticator::{AuthenticationStrategy, Credentials};
    use loom_infrastructure::config::CONFIG;
    use serde::{Deserialize, Serialize};

    use crate::Error;

    pub struct JwtAuthentication;

    impl AuthenticationStrategy for JwtAuthentication {
        type Error = Error;

        fn authenticate(&self, credentials: Credentials<'_>) -> Result<String, Self::Error> {
            let valid = verify(credentials.password, credentials.password_hash)
                .map_err(Error::BcryptError)?;

            if !valid {
                return Err(Error::InvalidCredentials);
            }

            let now = Utc::now().timestamp() as usize;
            let claims = Claims {
                sub: credentials.user_id.to_string(),
                email: credentials.email.to_string(),
                iat: now,
                exp: now + 86_400, // 24 hours
            };

            let secret = CONFIG.get_application().get_authentication_secret();
            encode(
                &Header::new(Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .map_err(Error::JwtError)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        /// Subject — the user's ID.
        sub: String,
        email: String,
        /// Issued-at timestamp (seconds since Unix epoch).
        iat: usize,
        /// Expiration timestamp (seconds since Unix epoch).
        exp: usize,
    }
}
