
pub mod db_request_handlers {
    use actix_web::{ web, HttpResponse};
    use redis::{RedisResult, RedisError};
    use serde::{Serialize, Deserialize};

    
    use crate::redis_tools::db_utils::{db_utils};

    

    #[derive(Serialize, Deserialize)]
    pub struct RedisPost {
        pub key: String,
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedisGet {
        pub key: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedisGetRespose{
        pub status: String,
        pub key: String,
        pub value: String,
    }
    //redis get handler
    pub async fn redis_get_handler(path: web::Path<RedisGet>) -> HttpResponse {
        
        log::info!("redis get handler");
        let key = path.key.clone();
        log::info!("key: {}", key);
        let result : RedisResult<String> = db_utils::fetch_from_db(&key);

        if result.is_err(){
            let result_err : RedisError = result.unwrap_err();
            log::error!("error: {}", result_err.to_string());
            log::error!("error {}", result_err.category());
            HttpResponse::Ok().json(RedisGetRespose {status: "error".to_owned(), key, value: "error".to_owned()})
        }else{
            let result_str: String = result.unwrap();
            log::info!("result: {}", result_str);
            HttpResponse::Ok().json(RedisGetRespose {status: "ok".to_owned(), key, value: result_str})
        }

    }

    pub async fn redis_post_handler(req: web::Json<RedisPost>) -> HttpResponse {
        log::info!("redis post handler");
        let key = req.key.clone();
        log::info!("key: {}", key);
        let value = req.value.clone();
        log::info!("value: {}", value);
        
        let result : RedisResult<String> = db_utils::set_to_db(&key, &value);

        if result.is_err(){
            let result_err : RedisError = result.unwrap_err();
            log::error!("error: {}", result_err.to_string());
            log::error!("error {}", result_err.category());
            HttpResponse::Ok().json(RedisGetRespose {status: "error".to_owned(), key, value: "error".to_owned()})
        }else{
            let result_str: String = result.unwrap();
            log::info!("result: {}", result_str);
            HttpResponse::Ok().json(RedisGetRespose {status: "ok".to_owned(), key, value: result_str})
        }
    }
}
