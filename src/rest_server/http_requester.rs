pub mod api_requests {
    use actix_web::{web, HttpResponse};
    use actix_web::client::Client;
    use http::header;

    use serde::{Serialize, Deserialize};
    use crate::spotify::spotify_structs::track::{Track};

    #[derive(Serialize,Deserialize)]
    pub struct ApiResponse {
        pub origin: String,
        pub url: String,
        headers: Headers,
    }
    
    #[derive(Serialize,Deserialize)]
    pub struct Headers{
        content_length: String,
        host: String,
        date: String,
        x_amzn_trace_id: String,
    }

    #[derive(Serialize,Deserialize)]
    pub struct SpotifyPath{
        token: String,
    }

    
    async fn get(link: &str) -> std::result::Result<String, ()>{

        //get ready for the http/https request
        let client = Client::build().disable_timeout().finish();
        
        let resp = client.get(link).send().await;

        //deal with the results of the request
        if resp.is_err(){
            let err = resp.err().unwrap();
            log::warn!("error encountered. that error was{}", err.to_string());

            Ok("error".to_owned())
        }else{
            let vec = resp.ok().unwrap().body().await.unwrap().to_vec();
            let response: String = String::from_utf8(vec.clone()).unwrap();
            Ok(response)
        }
        
    }
    async fn spotify_get(link: &str, token: &str) -> std::result::Result<String, ()>{
       

        log::info!("url: {}", link);

        //get ready for the http/https request
        let client = Client::build().disable_timeout().finish();
        let mut bearer_token: String = "Bearer ".to_owned();
        bearer_token.push_str(token);
        log::info!("Header {}: ", bearer_token);
        let resp = client.get(link)
        .header("Authorization", bearer_token)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Accept", "application/json")
        .send().await;
        

        //deal with the results of the request
        if resp.is_err(){
            let err = resp.err().unwrap();
            log::warn!("error encountered. that error was{}", err.to_string());

            Ok("error".to_owned())
        }else{
            let vec = resp.ok().unwrap().body().await.unwrap().to_vec();
            let response: String = String::from_utf8(vec.clone()).unwrap();
            Ok(response)
        }
        
    }

    pub async fn api_request() -> HttpResponse{
        let mut res: String = get("http://httpbin.org/get").await.unwrap();
        log::info!("response: {}", res);
        res = res.to_ascii_lowercase().replace("-", "_");
        
        let resp: ApiResponse = serde_json::from_str(&res).unwrap();
        HttpResponse::Ok().json(resp)
    }

    pub async fn spotify_test(path: web::Path<SpotifyPath>) -> HttpResponse{
        let token: String = path.token.clone();
        let mut res: String = spotify_get("https://api.spotify.com/v1/tracks/11dFghVXANMlKmJXsNCbNl?market=ES", &token).await.unwrap();
        log::info!("response: {}", res);
        res = res.to_ascii_lowercase().replace("-", "_");
        let resp: Track = serde_json::from_str(&res).unwrap();
        HttpResponse::Ok().json(resp)
    }
    
    
}

