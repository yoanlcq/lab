use std::fmt::{self, Display, Formatter};

// NOTE: Don't derive Debug, there's a password in there
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Credentials<T> {
    pub user: T,
    pub host: T,
    pub port: u32,
    pub database: T,
    pub password: T,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PublicCredentials<T> {
    pub user: T,
    pub host: T,
    pub port: u32,
    pub database: T,
}

impl<T: Display> Display for PublicCredentials<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{user}@{host}:{port} (database: {database})",
            user=self.user,
            host=self.host,
            port=self.port,
            database=self.database
        )
    }
}

impl<T> From<Credentials<T>> for PublicCredentials<T> {
    fn from(p: Credentials<T>) -> Self {
        let Credentials { host, user, database, port, password: _ } = p;
        Self { host, user, database, port }
    }
}
