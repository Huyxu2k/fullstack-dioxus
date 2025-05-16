

pub struct User {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_active: bool,
    created_at: chrono::NaiveDateTime,
}


#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get(&self, filter: FilterUsersRequest) -> Result<Vec<User>, RepoError>;
    async fn get_by_id(&self, id: i32) -> Result<User, RepoError>;
    async fn get_by_email_or_username(&self, email_or_username: String) -> Result<User, RepoError>;
    async fn create(&self, user: CreateUserRequest, by_id: i32) -> Result<User, RepoError>;
    async fn update(&self, id: i32, user: User) -> Result<User, RepoError>;
    async fn delete_by_id(&self, id: i32) -> Result<i32, RepoError>;
    async fn delete_list_ids(&self, id: Vec<i32>) -> Result<Vec<i32>, RepoError>;
}

