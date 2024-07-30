use sqlx::{postgres::PgTransactionManager, types::chrono::{DateTime, Local}, PgPool};

use crate::{database::PgTransaction, utils::password::hash_password, Result};

struct User{
    user_id: u64,
    username: String,
    email: String,
    create_date: Option<sqlx::types::chrono::DateTime<Local>>
}

impl User{

    async fn create_user(pool: &mut PgTransaction, username: String, email: String, password: String) -> Result<Self>{
        let hash_password = hash_password(&password)?;
 
        sqlx::query_as!(
            User,
            "insert into user_accounts(username, email)
            values(?,?)
            returning user_id, username, email, create_date
            ",
            username,
            email
        );
        

        Ok(Self {
            user_id: 2,
            username,
            email,
            create_date: None
        })
    }

}