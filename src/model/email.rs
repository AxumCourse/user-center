pub type ID = u64;

#[derive(Default)]
pub struct Email {
    pub id: ID,
    pub user_id: super::user::ID,
    pub email: String,
    pub is_primary: super::mysql::Bool,
    pub is_verify: super::mysql::Bool,
    pub is_del: super::mysql::Bool,
}
