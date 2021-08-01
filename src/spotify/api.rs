pub mod spotify_api{

    use actix_web::{web, HttpResponse};

    use serde::{Serialize, Deserialize};
    use crate::spotify::spotify_structs::track::{Track};
    use crate::spotify::algorithm::{algorithm};
    use crate::rest_server::http_requester::{api_requests};

    #[derive(Serialize,Deserialize)]
    pub struct SpotifyGenericPath{
        token: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NewTrackPost{
        last_track: String,
        current_track: String,
        preference: bool,
        uid: String,
        user_token: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NewTrackResp{
        last_track: String,
        next_track: String,
    }

    pub async fn spotify_generic(path: web::Path<SpotifyGenericPath>) -> HttpResponse{
        let token: String = path.token.clone();
        let mut res: String = api_requests::spotify_get("https://api.spotify.com/v1/tracks/11dFghVXANMlKmJXsNCbNl?market=ES", &token).await.unwrap();
        log::info!("response: {}", res);
        res = res.to_ascii_lowercase().replace("-", "_");
        let resp: Track = serde_json::from_str(&res).unwrap();
        HttpResponse::Ok().json(resp)
    }

    pub async fn next_track(req: web::Json<NewTrackPost>) -> HttpResponse{
        let last_track: String = algorithm::push_preference(req.last_track.clone(), req.current_track.clone(), req.preference.clone());
        let next_track: String = algorithm::get_recommended(last_track.clone());
        
        
        HttpResponse::Ok().json(NewTrackResp{
            last_track: last_track,
            next_track: next_track,
        })

    }
}