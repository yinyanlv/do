use std::env;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> MysqlConnection {

    // 加载.env文件
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");

    MysqlConnection::establish(&database_url).expect(&format!("Can not connect to {}", database_url))
}
