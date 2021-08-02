pub mod algorithm{

    use crate::redis_tools::db_utils::{db_utils};
    use crate::spotify::api::spotify_api;
    use crate::spotify::spotify_structs::{
        track::Track,
        artist::Artist,
        recommendations::Recommendations
    };
    
    use rand::Rng;
    

    struct DbEntry{
        trackid: String,
        weight: i32,
    }
    
    pub async fn get_recommended(trackid: String, token: String) -> Result<String, ()>{
        let mut db_entry: Vec<String>; 
        let mut loop_count: i32 = 0;
        let mut suggestions: Vec<DbEntry>;
        suggestions = Vec::new();
        loop {
            
            let temp = db_utils::get_zrandmember(&trackid);
            loop_count += 1;
            if temp.is_err(){
                db_entry = Vec::new();
                db_entry.push("none".to_owned());
                break;
            }
            db_entry = temp.unwrap();
            if suggestions.len() > 4{
                break;
            }

            if loop_count > 10{
                db_entry = Vec::new();
                db_entry.push("none".to_owned());
                break;
            }

            if db_entry[1].parse::<i32>().unwrap() < 0{
                continue;
            }else{
                suggestions.push(DbEntry {
                    trackid: db_entry[0].clone(),
                    weight: db_entry[1].parse::<i32>().unwrap(),
                });
            }
            
        }
        let mut trackid: String = "none".to_owned();
        let mut total_weight: i32 = 0;
        let mut new_vec: Vec<DbEntry> = Vec::new();
        if db_entry[0].eq("none"){
            //we need fallback to spotify recommended
            //get the artist of the track
            //get the genres of the artist
            //use the genre,artist, and track to get the top 5
            let res = spotify_api::get_spotify_track(token.clone(), trackid.clone());
            
            let track: Track = res.await;
            
            let artist: Artist = track.artists[0].clone();
            let artist_id: String = artist.id;
            let genres: Vec<String> = artist.genres.unwrap().clone();
            let genre: String = genres[0].clone();
            let recommendations: Recommendations = spotify_api::get_recommended(token.clone(), artist_id.clone(), trackid.clone(), genre.clone()).await;
            
            if trackid.eq("0"){
                Err(())
            }else{
                Ok(recommendations.tracks[0].id.clone())
            }
            
        }else{
            for entry in suggestions{
                total_weight += entry.weight;
                new_vec.push(entry);
            }
            
            let mut rng = rand::thread_rng();
            let mut random_number: i32 = rng.gen_range(0..total_weight);
            for entry in new_vec{
                random_number -= entry.weight;
                if random_number < 0{
                    trackid = entry.trackid.clone();
                    break;
                }
            }

            if trackid.eq("0"){
                Err(())
            }else{
                Ok(trackid)
            }
        }
    }

     pub fn push_preference(before_track: String, trackid: String, preference: bool) -> String{
         let inc: i32 = if preference {1}else{-1};
         let _res: Result<i32, redis::RedisError> = db_utils::increment_zset(&before_track, &trackid, inc);
         if preference{
             trackid
         }else{
             before_track
         }
     }

     

     

     
}