
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