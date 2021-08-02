
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

    pub fn get_zset_rank(key: &String, item: &String) -> Result<u32, redis::RedisError>{
        log::info!("fetching from db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<u32> = redis::cmd("GET").arg(key).arg(item).query(&con);
        if result.is_err(){
            return Result::Ok(0);
        }else{
            return Result::Ok(result.unwrap());
        }
    }

    pub fn set_zset_rank(key: &String, item: &String, rank: u32) -> Result<u32, redis::RedisError>{
        log::info!("setting to db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<u32> = redis::cmd("ZADD").arg(key).arg(item).arg(rank).query(&con);
        if result.is_err(){
            return Result::Ok(rank);
        }else{
            return Result::Ok(result.unwrap());
        }
    }

    pub fn increment_zset(key: &String, item: &String, inc: i32) -> Result<i32, redis::RedisError>{
        log::info!("setting to db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<i32> = redis::cmd("ZINCRBY").arg(key).arg(inc).arg(item).query(&con);
        if result.is_err(){
            return Result::Ok(inc);
        }else{
            return Result::Ok(result.unwrap());
        }
    }

    pub fn get_zrandmember(key: &String) -> Result<Vec<String>, redis::RedisError>{
                log::info!("fetching from db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<Vec<String>> = redis::cmd("ZRANDMEMBER").arg(key).arg("1").arg("WITHSCORES").query(&con);
        if result.is_err(){
            let mut vec: Vec<String> = Vec::new();
            vec.push("none".to_owned());
            return Result::Ok(vec);
        }else{
            return Result::Ok(result.unwrap());
        }
    }

    
    pub fn set_to_db(key: &String, value: &String ) -> redis::RedisResult<String>{
        log::info!("fetching from db");
        let client = redis::Client::open("redis://10.0.249.54").unwrap();
        let con = client.get_connection()?;
        let result : RedisResult<String> = redis::cmd("SET").arg(key).arg(value).query(&con);
        
        result
    }
}

#[cfg(test)]
mod tests {
    extern crate redis;
    
    use super::db_utils;
    use redis::{RedisResult, ErrorKind};

    #[test]
    fn fetch_from_db(){
        let key: String = "this_shouldnt_exist".to_owned();
        
        let result: RedisResult<String> = db_utils::fetch_from_db(&key);
        let error: ErrorKind = result.err().unwrap().kind();
        let ne : bool = error == ErrorKind::IoError;
        let e : bool = error == ErrorKind::TypeError;
        assert_eq!((ne || e), true);
    }

    #[test]
    fn set_to_db(){
        let key: String = "test_key".to_owned();
        let value: String = "test_value".to_owned();
        
        let result: RedisResult<String> = db_utils::set_to_db(&key, &value);
        if result.is_err(){
            let error: ErrorKind = result.err().unwrap().kind();
            let e : bool = error == ErrorKind::IoError;
            assert_eq!(e, true);
        }else{
            assert_eq!(result.unwrap(), "OK");
        }

    }
}