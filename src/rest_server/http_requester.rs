pub mod api_requests {
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

    
    async fn get(link: &str) -> Result<String, ()>{
        //create a dns resolver
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

        //get the protocol from the url as arg
        let split_proto = link.split("//").collect::<Vec<&str>>();

        //split on "/"
        let split = split_proto[1].split("/").collect::<Vec<&str>>();

        //get the ip address of the url
        let ip_addr = resolver.lookup_ip(split[0]).unwrap().iter().next().unwrap().to_string();
        
        //create the starting of the url
        let mut url: String = split_proto[0].to_owned();
        url.push_str("//");
        url.push_str(&ip_addr[..]);

        //add all the missing pieces/parts of the url
        for s in split{
            url.push_str("/");
            url.push_str(s);
        }

        //get ready for the http/https request
        let client = Client::default();
        let resp = client.get(url).send().await;

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
        let res: String = get("http://httpbin.org/get").await.unwrap();
        HttpResponse::Ok().json( res)
    }
    
    
}

