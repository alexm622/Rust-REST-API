pub mod spotify_api{

    use actix_web::{web, HttpResponse};

    use serde::{Serialize, Deserialize};
    use crate::spotify::spotify_structs::track::{Track};
    use crate::rest_server::http_requester::{api_requests};

    #[derive(Serialize,Deserialize)]
    pub struct SpotifyGenericPath{
        token: String,
    }

    pub async fn spotify_generic(path: web::Path<SpotifyGenericPath>) -> HttpResponse{
        let token: String = path.token.clone();
        let mut res: String = api_requests::spotify_get("https://api.spotify.com/v1/tracks/11dFghVXANMlKmJXsNCbNl?market=ES", &token).await.unwrap();
        log::info!("response: {}", res);
        res = res.to_ascii_lowercase().replace("-", "_");
        let resp: Track = serde_json::from_str(&res).unwrap();
        HttpResponse::Ok().json(resp)
    }
}