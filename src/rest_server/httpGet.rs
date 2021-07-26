pub mod api_requests {
    extern crate reqwest; // 0.9.18
    use actix_web::{ HttpResponse};
    use actix_web::client::Client;
    use trust_dns_resolver::Resolver;
    use trust_dns_resolver::config::*;
    
    use serde::{Deserialize};

    #[derive(Deserialize)]
    pub struct RestRequest {
        pub origin: String,
        pub url: String,
    }

    //this works
    //and I dont wanna know how
    //need to do an exorcism on this function later
    //probably write a test for this function later buuut
    //no
    async fn get() -> Result<String, ()>{
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
        let ip_addr = resolver.lookup_ip("httpbin.org").unwrap().iter().next().unwrap().to_string();
        let mut url: String = "http://".to_owned();
        url.push_str(&ip_addr[..]);
        url.push_str("/get");
        let client = Client::default();
        let resp = client.get(url)
            .send().await
            .ok().unwrap().body()
            .await.unwrap().to_vec();
        let response: String = String::from_utf8(resp.clone()).unwrap();
        Ok(response)
    }

    pub async fn api_request() -> HttpResponse{
        let res: String = get().await.unwrap();
        HttpResponse::Ok().json( res)
    }
    
    
}

