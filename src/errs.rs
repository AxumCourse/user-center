use axum::response::IntoResponse;

#[derive(Debug)]
pub enum ErrorKind {
    Db,
    Mq,
    Smtp,
    Cfg,
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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        format!("{:?}", self).into_response()
    }
}
