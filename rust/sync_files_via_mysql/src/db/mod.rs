pub mod mysql;
pub use mysql::*;
pub mod credentials;
pub use credentials::*;

#[derive(Debug, Hash)]
pub struct Db {
    pub mysql: MySql,
    pub credentials: PublicCredentials<String>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct DbInfo {
    pub mysql_client_version: String,
}

impl Db {
    pub fn info() -> DbInfo {
        DbInfo {
            mysql_client_version: MySql::client_version(),
        }
    }
    pub fn new(p: &Credentials<String>) -> Result<Self, String> {
        use ::std::ffi::CString;
        let c = Credentials {
            port: p.port,
            host    : CString::new(p.host    .as_str()).unwrap(),
            user    : CString::new(p.user    .as_str()).unwrap(),
            database: CString::new(p.database.as_str()).unwrap(),
            password: CString::new(p.password.as_str()).unwrap(),
        };
        let mysql = MySql::new(&c)?;
        info!("Connected to {}", PublicCredentials::from(p.clone()));
        Ok(Self { mysql, credentials: p.clone().into() })
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        info!("Closed connection to {}", self.credentials);
    }
}
