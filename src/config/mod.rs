use std::env;


#[derive(Debug,Clone)]
pub struct Config {
    pub db_url : String,
    pub jwt_secret : String,
    pub qr_secret : String,
    pub jwt_maxage : i64,
    pub port : u32,

    pub aws_access_key: String,
    pub aws_secret_key: String,
    pub aws_region: String,
    pub s3_bucket: String,
}

impl Config {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("Database Url does not exist");
        let jwt_secret = env::var("jwt_secret").expect("jwt secret does not exist");
        let qr_secret = env::var("qr_secret").expect("qr secret does not exist");
        let jwt_maxage = env::var("jwt_maxage").expect("jwt expiration time does not exist");
        let port = env::var("port")
                                            .unwrap_or_else(|_|{"8080".to_string()})
                                            .parse::<u32>()
                                            .expect("port number must be a u32");

         let aws_access_key = env::var("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID missing");

        let aws_secret_key = env::var("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY missing");

        let aws_region = env::var("AWS_REGION")
            .expect("AWS_REGION missing");

        let s3_bucket = env::var("S3_BUCKET")
            .expect("S3_BUCKET missing");

        Self { 
            db_url: database_url, 
            jwt_secret,
            qr_secret,
            jwt_maxage:jwt_maxage.parse::<i64>().expect("expiration must be a i16"), 
            port,
            aws_access_key,
            aws_secret_key,
            aws_region,
            s3_bucket,
        }
    }
}