pub mod test_lifecycle {
    pub fn before() {
        dotenvy::from_filename_override(".env.test").expect("Failed to load .env.test.");
    }

    pub fn after() {
        dotenvy::from_filename_override(".env.dev").ok();
    }
}
