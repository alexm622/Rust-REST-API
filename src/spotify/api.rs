
pub mod spotify_api{

    use actix_web::{web, HttpResponse};

    use serde::{Serialize, Deserialize};
    use crate::spotify::spotify_structs::{
        track::Track,
        artist::Artist,
        recommendations::Recommendations
    };
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
        log::info!("requesting generic spotify data");
        let token: String = path.token.clone();
        let mut res: String = api_requests::spotify_get("https://api.spotify.com/v1/tracks/11dFghVXANMlKmJXsNCbNl?market=US", &token).await.unwrap();
        log::info!("response: {}", res);
        res = res.replace("-", "_");
        log::info!("got a response of {}", res);
        let resp: Track = serde_json::from_str(&res).unwrap();
        HttpResponse::Ok().json(resp)
    }

    pub async fn get_spotify_track(token: String, track: String) -> Track{
        log::info!("requesting track");
        let url: String = "https://api.spotify.com/v1/tracks/&?market=US".to_owned().replace("&", &track);
        log::info!("requesting from url:{}", url.clone());
        let mut res: String = api_requests::spotify_get(&url, &token).await.unwrap();
        res = res.replace("-", "_");
        log::info!("got a response of {}", res);
        let resp: Track = serde_json::from_str(&res).unwrap();
        resp
    }

    pub async fn get_spotify_artist(token: String, artist: String) -> Artist{
        let url: String = "https://api.spotify.com/v1/artists/&?market=US".to_owned().replace("&", &artist);
        log::info!("getting an artists data");
        log::info!("requesting from url:{}", url.clone());
        let mut res: String = api_requests::spotify_get(&url, &token).await.unwrap();
        res = res.replace("-", "_");
        log::info!("got a response of {}", res);
        let resp: Artist = serde_json::from_str(&res).unwrap();
        resp
    }

    pub async fn get_recommended(token: String, artist: String, trackid: String, genre: String) -> Recommendations{
        let url: String = "https://api.spotify.com/v1/recommendations?limit=10&seed_artists=^&seed_genres=*&seed_tracks=%&market=US".to_owned()
            .replace("*", &genre).replace("^", &artist).replace("%", &trackid).replace(" ", "%20");
        log::info!("getting recommended track info");
        log::info!("requesting from url:{}", url.clone());
        let mut res: String = api_requests::spotify_get(&url, &token).await.unwrap();
        res = res.replace("-", "_");
        log::info!("got a response of {}", res);
        let resp: Recommendations = serde_json::from_str(&res).unwrap();
        resp
    }

    
    

    pub async fn next_track(req: web::Json<NewTrackPost>) -> HttpResponse{
        log::info!("data was posted successfully");
        let last_track: String = algorithm::push_preference(req.last_track.clone(), req.current_track.clone(), req.preference.clone());
        let next_track: String = algorithm::get_recommended(last_track.clone(), req.user_token.clone()).await.unwrap();
        
        
        HttpResponse::Ok().json(NewTrackResp{
            last_track: last_track,
            next_track: next_track,
        })

    }
}