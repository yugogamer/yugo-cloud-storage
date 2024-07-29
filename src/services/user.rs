use sqlx::{types::chrono::{DateTime, Local}, PgPool};

struct User{
    user_id: u64,
    username: String,
    email: String,
    create_date: Option<sqlx::types::chrono::DateTime<Local>>
}

impl User{

    async fn create_user(username: String, email: String, password: String) -> Self{
        Self {
            user_id: 2,
            username,
            email,
            create_date: None
        }
    }

}

/*
async fn create_user(pool: &PgPool, username: String, email: String, password: String) -> Result<String, String>[
    Ok("".into())
}
    */