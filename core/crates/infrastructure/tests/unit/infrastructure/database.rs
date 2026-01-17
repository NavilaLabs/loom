use infrastructure::database::{Error, Pool, ScopeDefault, StateConnected};
use url::Url;

async fn get_default_postgres_pool() -> Result<Pool<ScopeDefault, StateConnected>, Error> {
    let database_url = "postgres://postgres:postgres@postgres:5432/postgres";
    Pool::connect(&Url::parse(database_url).unwrap()).await
}

#[tokio::test]
async fn test_connect_to_postgres_database() {
    assert!(get_default_postgres_pool().await.is_ok());
}
