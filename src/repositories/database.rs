pub struct Config {
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_database: String,
}

impl Config {
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

