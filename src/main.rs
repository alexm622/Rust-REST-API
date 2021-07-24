use actix_web::{post, App, HttpServer, web, HttpResponse, Responder, get};
use serde::{Serialize, Deserialize};
extern crate simple_logger;
extern crate redis;

use redis::{Commands, RedisError, RedisResult, ConnectionLike, FromRedisValue};


#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
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



// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/posttest", web::post().to(post_test_handler));
    
}

//post request handler
pub async fn post_test_handler (req: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.name.to_string())
}
//Configure handler
pub async fn health_check_handler() ->  impl Responder {
    HttpResponse::Ok().json("recieved")
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


