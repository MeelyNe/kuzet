use std::env;
use dotenv::dotenv;

pub struct ServerConfig {
    pub port: u16,
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: env::var("PORT").
                expect("PORT must be set").
                parse().
                expect("PORT must be a number"),
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig::default()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            host: env::var("DB_HOST").
                expect("DB_HOST must be set"),
            port: env::var("DB_PORT").
                expect("DB_PORT must be set").
                parse().
                expect("DB_PORT must be a number"),
            user: env::var("DB_USER").
                expect("DB_USER must be set"),
            password: env::var("DB_PASSWORD").
                expect("DB_PASSWORD must be set"),
            database: env::var("DB_NAME").
                expect("DB_NAME must be set"),
        }
    }
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::new(),
            database: DatabaseConfig::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        dotenv().expect("Failed to read .env file");

        Config::default()
    }
}