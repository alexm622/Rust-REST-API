pub mod algorithm{

    use crate::redis_tools::db_utils::{db_utils};
    

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
        let mut trackid: String = "non-existant".to_owned();
        let mut total_weight: i32 = 0;
        let mut new_vec: Vec<DbEntry> = Vec::new();
        
        for entry in suggestions{
            total_weight += entry.weight;
            new_vec.push(entry);
        }
        let mut random_number: i32 = 0;
        
        for entry in new_vec{
            random_number -= entry.weight;
            if random_number < 0{
                trackid = entry.trackid.clone();
                break;
            }
        }
        trackid            
     }
}