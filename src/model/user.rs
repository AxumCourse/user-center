pub type ID = u64;

pub type Status = i8;
pub const STATUS_PENDING: Status = 0;
pub const STATUS_OK: Status = 1;
pub const STATUS_LOCKED: Status = 2;

#[derive(Default)]
pub struct User {
    pub id: ID,
    pub username: String,
    pub email: String,
    pub password: String,
    pub dateline: super::mysql::Dateline,
    pub status: Status,
    pub is_del: super::mysql::Bool,
    pub emails: Vec<super::Email>,
}
