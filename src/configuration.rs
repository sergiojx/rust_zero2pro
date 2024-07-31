use config;


#[derive(serde::Deserialize)] 
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn configuration_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
        self.username,self.password,self.host,self.port,self.database_name)
    }

    pub fn configuration_string_without_db(&self) -> String {
        // Omitting the database name we connect to the Postgres instance,
        // not a specific logicak database
        format!("postgres://{}:{}@{}:{}",
        self.username,self.password,self.host,self.port)
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise a configuration reader
    let settings = config::Config::builder()
        // Add configuration values from file `configuration.yaml`
        .add_source(config::File::new("/vagrant/zerotoproduction/zero2prod/src/configuration.yaml", config::FileFormat::Yaml)).build()?;
    // Try to convert the configuration values it reads into
    // our Settings type
    settings.try_deserialize::<Settings>()
}

