pub mod algorithm{

    use crate::redis_tools::db_utils::{db_utils};

    use rand::Rng;
    

    struct DbEntry{
        trackid: String,
        weight: i32,
    }

    pub fn get_recommended(trackid: String) -> String{
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
        }
        
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

        trackid            
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