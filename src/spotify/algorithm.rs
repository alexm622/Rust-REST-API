pub mod algorithm{

    use crate::redis_tools::db_utils::{db_utils};

    pub fn get_recommended(trackid: String) -> String{
        let mut db_entry: Vec<String>; 
        let mut loop_count: i32 = 0;
        loop {
            loop_count += 1;
            db_entry = db_utils::get_zrandmember(&trackid).unwrap();
            if db_entry.len() < 2{
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
                break;
            }
            
        }
        let trackid: String;
        if db_entry.len() < 2{
            //get from spotify
            trackid = "hah".to_owned();

        }else{
            trackid = db_entry[0].parse::<String>().unwrap();
        }
        trackid            
     }
}