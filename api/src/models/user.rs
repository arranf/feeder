use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32;
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime, // only NaiveDateTime works here due to diesel limitations
}

impl User {
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }
}