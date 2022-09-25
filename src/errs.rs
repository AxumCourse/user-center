use axum::response::IntoResponse;

#[derive(Debug)]
pub enum ErrorKind {
    Db,
    Mq,
    Smtp,
    Cfg,
    AlreadyExists,
    Bcrypt,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self { kind, msg, cause }
    }
    pub fn from_err(kind: ErrorKind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }
    pub fn from_str(kind: ErrorKind, msg: &str) -> Self {
        Self::new(kind, msg.to_string(), None)
    }
    pub fn already_exists(msg: &str) -> Self {
        Self::from_str(ErrorKind::AlreadyExists, msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Self::from_err(ErrorKind::Cfg, Box::new(err))
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::from_err(ErrorKind::Db, Box::new(err))
    }
}
impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::from_err(ErrorKind::Bcrypt, Box::new(err))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        format!("{:?}", self).into_response()
    }
}
