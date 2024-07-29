use rocket::{fairing::{self, AdHoc}, Build, Rocket};
use s3::{creds::Credentials, Bucket, Region};

pub fn load_bucket() -> AdHoc{
    AdHoc::on_ignite("S3 connection", |rocket| async {
        rocket.attach(AdHoc::try_on_ignite("connect to s3", connect_to_s3))
    })
}

async fn connect_to_s3(rocket: Rocket<Build>) -> fairing::Result{
    let access_key = std::env::var("ACCESS_KEY").unwrap_or("".into());
    let secret_key = std::env::var("SECRET_KEY").unwrap_or("".into());

    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap();

    let region_name = std::env::var("REGION_NAME").unwrap_or("".into());
    let endpoint = std::env::var("ENDPOINT").unwrap_or("".into());

    let region = Region::Custom { region: region_name, endpoint };
    
    let bucket_name = std::env::var("BUCKET_NAME").unwrap_or("".into());

    match Bucket::new(&bucket_name, region, credentials){
        Ok(bucket) => {
            match bucket.list("".to_string(), None).await{
                Ok(_) => Ok(rocket),
                Err(err) => {
                    error!("no connection to bucket : {err}");
                    Err(rocket)
                },
            }
        },
        Err(err) => {
            error!("can't access bucket: {err}");
            Err(rocket)
        },
    }
}