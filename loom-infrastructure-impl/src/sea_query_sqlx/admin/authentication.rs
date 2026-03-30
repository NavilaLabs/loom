use loom_core::admin::authenticator::AuthenticationStrategy;

pub mod jwt {
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use loom_core::admin::authenticator::AuthenticationStrategy;
    use loom_infrastructure::config::CONFIG;
    use serde::{Deserialize, Serialize};

    pub struct JwtAuthentication;

    impl AuthenticationStrategy for JwtAuthentication {
        type Error = crate::Error;

        fn authenticate<String>(
            &self,
            secret: &str,
            name: Option<&str>,
        ) -> Result<Option<String>, Self::Error> {
            // let token = encode(
            //     &Header::default(),
            //     todo!(),
            //     &EncodingKey::from_secret(
            //         CONFIG
            //             .get_application()
            //             .get_authentication_secret()
            //             .as_ref(),
            //     ),
            // );

            todo!()
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        sub: String,
        company: String,
        exp: usize,
    }
}
