pub mod algorithm{

    use crate::redis_tools::db_utils::{db_utils};
    use crate::spotify::api::spotify_api;
    use crate::spotify::spotify_structs::{
        track::Track,
        artist::Artist,
        recommendations::Recommendations
    };
    
    use rand::Rng;
    use redis::{RedisError};
    

    struct DbEntry{
        trackid: String,
        weight: i32,
    }
    
    pub async fn get_recommended(trackid: String, token: String) -> Result<String, ()>{
        let mut db_entry: Vec<String>; 
        let mut loop_count: i32 = 0;
        let mut suggestions: Vec<DbEntry>;
        suggestions = Vec::new();
        log::info!("starting algorithm loop");
        log::info!("trackid is {}", trackid.clone());
        let count = db_utils::get_zcount(&trackid).unwrap();
        
        loop {
            
            let temp = db_utils::get_zrandmember(&trackid);
            loop_count += 1;
            db_entry = temp.unwrap();
            if db_entry.len() == 0{
                log::warn!("nothing was found in the database");
                db_entry = Vec::new();
                db_entry.push("none".to_owned());
                break;
            }
            
            if suggestions.len() > count as usize{
                log::info!("max num of suggestions found");
                break;
            }

            if loop_count > 5{
                log::info!("max num of iterations done");
                
                break;
            }

            log::info!("db_entry: {}", db_entry[0].clone());

            if db_entry[1].parse::<i32>().unwrap() < 1{
                log::info!("db entry of: {} did not have a high enough weight", db_entry[0].clone());
                continue;
            }else{
                log::info!("adding trackid {} with weight {}",  db_entry[0].clone(), db_entry[1].clone());
                suggestions.push(DbEntry {
                    trackid: db_entry[0].clone(),
                    weight: db_entry[1].parse::<i32>().unwrap(),
                });
            }
            
        }
        
        let mut total_weight: i32 = 0;
        let mut new_trackid: String;
        let mut new_vec: Vec<DbEntry> = Vec::new();
        if db_entry[0].eq("none"){
            log::info!("falling back to spotify recommended");
            //we need fallback to spotify recommended
            //get the artist of the track
            //get the genres of the artist
            //use the genre,artist, and track to get the top 5
            let res = spotify_api::get_spotify_track(token.clone(), trackid.clone());
            
            let track: Track = res.await;
            let artist_id: String = track.artists[0].clone().id;
            let artist: Artist = spotify_api::get_spotify_artist(token.clone(), artist_id.clone()).await;
            let genres: Vec<String> = artist.genres.unwrap().clone();
            let genre: String = genres[0].clone();
            let recommendations: Recommendations = spotify_api::get_recommended(token.clone(), artist_id.clone(), trackid.clone(), genre.clone()).await;
            for recommendation  in recommendations.clone().tracks{
                //dump reccomend into db
                let _res = db_utils::set_zset_rank(&trackid, &recommendation.id, 1);          
            }
            
            if trackid.eq("0"){
                Err(())
            }else{
                Ok(recommendations.tracks[0].id.clone())
            }
            
        }else{
            log::info!("looping through suggestions");
            for entry in suggestions{
                total_weight += entry.weight;
                new_vec.push(entry);
            }
            
            let mut rng = rand::thread_rng();
            let mut random_number: i32 = rng.gen_range(0..total_weight);
            new_trackid = "0".to_owned();
            for entry in new_vec{
                random_number -= entry.weight;
                if random_number < 0{
                    new_trackid = entry.trackid.clone();
                    break;
                }
            }

            if trackid.eq("0"){
                Err(())
            }else{
                Ok(new_trackid)
            }
        }
    }

     pub fn push_preference(before_track: String, trackid: String, preference: bool) -> String{
         log::info!("pushing preference");
         let inc: i32 = if preference {1}else{-1};
         let res: Result<i32, redis::RedisError> = db_utils::increment_zset(&before_track, &trackid, inc);
         if res.is_err() {
             log::error!("pushing to db failed");
             let result_err : RedisError = res.unwrap_err();
             log::error!("error: {}", result_err.to_string());
             log::error!("error {}", result_err.category());
         }
         if preference{
             trackid
         }else{
             before_track
         }
     }

     

     

     
}