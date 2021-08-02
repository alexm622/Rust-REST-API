use actix_web::{ App, HttpServer, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

extern crate simple_logger;
use simple_logger::{SimpleLogger};
use log::LevelFilter;



pub mod rest_server{
    pub mod db_tools;
    pub mod http_requester;
}
pub mod redis_tools{
    pub mod db_utils;
}
pub mod spotify{
    pub mod spotify_structs;
    pub mod algorithm;
    pub mod api;
}
pub mod utils{
}



use rest_server::db_tools::{db_request_handlers};
use rest_server::http_requester::{api_requests};
use spotify::api::{spotify_api};


#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
}

// Configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/posttest", web::post().to(post_test_handler));
    
    cfg.service(web::resource("/redis&key={key}").route(web::get().to(db_request_handlers::redis_get_handler)));
    cfg.route("/redis", web::post().to(db_request_handlers::redis_post_handler));
    cfg.route("/apitest", web::get().to(api_requests::api_request));
    cfg.service(web::resource("/spotify&token={token}").route(web::get().to(spotify_api::spotify_generic)));
}

//post request handler
pub async fn post_test_handler (req: web::Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json(req.name.to_string())
}
//Configure handler
pub async fn health_check_handler() ->  impl Responder {
    HttpResponse::Ok().json("Rust Server is running properly")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    

    SimpleLogger::new()
    .with_level(LevelFilter::Off)
    .with_module_level("rest", LevelFilter::Info)
    .with_module_level("actix", LevelFilter::Info)
    .init()
    .unwrap();


    
    HttpServer::new(|| {
        App::new()
            .configure(general_routes)
    })
        .workers(20)
        .keep_alive(15)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


