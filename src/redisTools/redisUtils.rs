
pub mod db_utils{
    extern crate simple_logger;
    extern crate redis;

    use redis::{RedisResult};

    pub fn fetch_from_db(key: &String ) -> redis::RedisResult<String>{
        log::info!("fetching from db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<String> = redis::cmd("GET").arg(key).query(&con);
         
        result
    }
    
    pub fn set_to_db(key: &String, value: &String ) -> redis::RedisResult<String>{
        log::info!("fetching from db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<String> = redis::cmd("SET").arg(key).arg(value).query(&con);
        
        result
    }
}