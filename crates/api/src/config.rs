use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Render provides PORT, fall back to API_PORT, then default to 3000
        let port = env::var("PORT")
            .or_else(|_| env::var("API_PORT"))
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;

        let host = env::var("API_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        Ok(Config {
            database_url,
            port,
            host,
        })
    }
}
