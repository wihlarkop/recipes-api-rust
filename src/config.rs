use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
}
