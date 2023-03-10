use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(long, env = "PORT", default_value_t = 8080)]
    pub port: u16,

    #[clap(long, env = "DATABASE_URL", default_value = "sqlite://db.sqlite3")]
    pub database_url: String,

    #[clap(long, env = "AUTH")]
    pub auth: bool,
}
