pub struct Config {
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_database: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            postgres_host: std::env::var("DB_HOST").unwrap(),
            postgres_port: std::env::var("DB_PORT").unwrap(),
            postgres_user: std::env::var("DB_USER").unwrap(),
            postgres_password: std::env::var("DB_PASSWORD").unwrap(),
            postgres_database: std::env::var("DB_NAME").unwrap(),
        }
    }

    pub fn to_dsn(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}

