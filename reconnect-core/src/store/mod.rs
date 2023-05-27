use crate::config::TableConfig;
use crate::store::postgres_store::PostgresStore;
use lazy_static::lazy_static;
use tera::Tera;

pub mod postgres_store;

lazy_static! {
    pub static ref SQL_TEMPLATES: Tera = {
        let mut tera = Tera::new("../templates/**/*").unwrap();
        tera.autoescape_on(vec![]);
        tera
    };
}

enum StoreType {
    Postgres,
    //TODO - Add more store types
}
pub fn get_store(config: &TableConfig) -> anyhow::Result<PostgresStore> {
    //TODO - Params is None
    //println!("Connection URI : {}", config.connection_uri);
    let dsn = dsn::parse(config.connection_uri.as_str())?;
    let host = dsn.host.expect("Unable to parse dsn.host from connection_uri");
    let port = dsn.port.expect("Unable to parse dsn.port from connection_uri");
    let username = dsn.username.expect("Unable to parse dsn.username from connection_uri");
    let password = dsn.password.expect("Unable to parse dsn.password from connection_uri");
    let database = dsn.database.expect("Unable to parse dsn.database from connection_uri");

    let store = PostgresStore::new(host, port, username, password, database, None);
    Ok(store)
}
