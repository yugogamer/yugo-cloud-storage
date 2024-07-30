use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use sqlx::{PgPool, Postgres, Transaction};

pub type PgTransaction = Transaction<'static, Postgres>;

pub fn load_db() -> AdHoc {
    AdHoc::on_ignite("Postgres pool Stage", |rocket| async {
        rocket
        .attach(AdHoc::try_on_ignite("connect to postgres", connect))
        .attach(AdHoc::try_on_ignite("running migration", migrate))
    })
}

async fn connect(rocket: Rocket<Build>) -> fairing::Result {
    let url = std::env::var("DATABASE_URL").unwrap_or("postgres://api_user:api_password@localhost:5432/files".into());
    match PgPool::connect(&url).await {
        Ok(pool) => {
            Ok(rocket.manage(pool))
        }
        Err(err) => {
            error!("Can't connect to db : {err}");
            Err(rocket)
        }
    }
}

async fn migrate(rocket: Rocket<Build>) -> fairing::Result{
    let pool = &rocket.state::<PgPool>();
    if let Some(pool) = pool {
        match sqlx::migrate!("./migrations").run(&**pool).await{
            Ok(_) => {
                Ok(rocket)
            }
            Err(err) => {
                error!("Error during migration: {err}");
                Err(rocket)
            }
        }   
    }else {
        error!("No db for migration");
        Err(rocket)
    }
}