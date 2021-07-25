use actix_web::{post, App, HttpServer, web, HttpResponse, Responder, get};
use serde::{Serialize, Deserialize};
extern crate simple_logger;
extern crate redis;

use redis::{Commands, RedisError, RedisResult, ConnectionLike, FromRedisValue};


#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct redisPost {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct redisGet {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct redisGetRespose{
    pub status: String,
    pub key: String,
    pub value: String,
}

fn do_something() -> redis::RedisResult<()>{
    log::info!("doing something");
    let client = redis::Client::open("redis://10.0.249.54").unwrap();
    let con = client.get_connection()?;
    let result : RedisResult<String> = redis::cmd("GET").arg("foobar").query(&con);
    
    if result.is_err(){
        let resultErr : RedisError = result.unwrap_err();
        log::error!("error: {}", resultErr.to_string());
        log::error!("error {}", resultErr.category());
        
    }else{
        log::info!("result: {}", result.unwrap());
    }


    Ok(())
}

fn fetch_from_db(key: &String ) -> redis::RedisResult<String>{
    log::info!("fetching from db");
    let client = redis::Client::open("redis://10.0.249.54").unwrap();
    let con = client.get_connection()?;
    let result : RedisResult<String> = redis::cmd("GET").arg(key).query(&con);
     
    result
}

fn set_to_db(key: &String, value: &String ) -> redis::RedisResult<String>{
    log::info!("fetching from db");
    let client = redis::Client::open("redis://10.0.249.54").unwrap();
    let con = client.get_connection()?;
    let result : RedisResult<String> = redis::cmd("SET").arg(key).arg(value).query(&con);
    
    result
}




// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/posttest", web::post().to(post_test_handler));
    
    cfg.service(web::resource("/redis&key={key}").route(web::get().to(redis_get_handler)));
    cfg.route("/redis", web::post().to(redis_post_handler));
    
}

//post request handler
pub async fn post_test_handler (req: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.name.to_string())
}
//Configure handler
pub async fn health_check_handler() ->  impl Responder {
    HttpResponse::Ok().json("Rust Server is running properly")
}

//redis get handler
pub async fn redis_get_handler(path: web::Path<redisGet>) -> HttpResponse {
    
    log::info!("redis get handler");
    let key = path.key.clone();
    log::info!("key: {}", key);
    let result : RedisResult<String> = fetch_from_db(&key);

    if result.is_err(){
        let resultErr : RedisError = result.unwrap_err();
        log::error!("error: {}", resultErr.to_string());
        log::error!("error {}", resultErr.category());
        HttpResponse::Ok().json(redisGetRespose {status: "error".to_owned(), key, value: "error".to_owned()})
    }else{
        let result_str: String = result.unwrap();
        log::info!("result: {}", result_str);
        HttpResponse::Ok().json(redisGetRespose {status: "ok".to_owned(), key, value: result_str})
    }

}

pub async fn redis_post_handler(req: web::Json<redisPost>) -> HttpResponse {
    log::info!("redis post handler");
    let key = req.key.clone();
    log::info!("key: {}", key);
    let value = req.value.clone();
    log::info!("value: {}", value);
    
    let result : RedisResult<String> = set_to_db(&key, &value);

    if result.is_err(){
        let resultErr : RedisError = result.unwrap_err();
        log::error!("error: {}", resultErr.to_string());
        log::error!("error {}", resultErr.category());
        HttpResponse::Ok().json(redisGetRespose {status: "error".to_owned(), key, value: "error".to_owned()})
    }else{
        let result_str: String = result.unwrap();
        log::info!("result: {}", result_str);
        HttpResponse::Ok().json(redisGetRespose {status: "ok".to_owned(), key, value: result_str})
    }
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger:: init_with_level(log::Level::Info).unwrap();
    do_something();
    HttpServer::new(|| {
        App::new()
            .configure(general_routes)
    })
        .workers(10)
        .keep_alive(15)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


