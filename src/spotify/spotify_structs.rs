pub mod recommendations {
    use serde::{Deserialize, Serialize};

    use super::track::Track;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Recommendations{
        pub tracks: Vec<Track>,
    }    
}


pub mod related_artists {
    use super::artist::Artist;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RelatedArtists {
        pub artists: Vec<Artist>,
    }

    
}

pub mod track {
    use serde::{Deserialize, Serialize};

    use super::artist::Artist;
    use super::album::Album;
    use super::repeated_use::{ExternalUrls, ExternalIds};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Track {
        pub album: Option<Album>,
        pub artists: Vec<Artist>,
        pub disc_number: i32,
        pub duration_ms: u32,
        pub explicit: bool,
        pub external_urls: ExternalUrls,
        pub external_ids: ExternalIds,
        pub href: String,
        pub id: String,
        pub is_local: bool,
        pub name: String,
        pub preview_url: Option<String>,
        pub track_number: i32,
        pub uri: String,
    }

    

}

pub mod album {
    use serde::{Deserialize, Serialize};
    use super::repeated_use::{ExternalUrls, Image};
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Album {
        pub external_urls: ExternalUrls,
        pub href: String,
        pub id: String,
        pub images: Vec<Image>,
        pub name: String,
        pub uri: String,
    }
}

pub mod artist {
    use serde::{Deserialize, Serialize};
    
    use super::repeated_use::{ExternalUrls, Followers, Image};
    
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Artist {
        pub external_urls: ExternalUrls,
        pub href: String,
        pub id: String,
        pub name: String,
        pub uri: String,
        pub genres:  Option<Vec<String>>,
        pub followers: Option<Followers>,
        pub images: Option<Vec<Image>>,
    }


}

pub mod repeated_use{
    use serde::{Deserialize, Serialize};

    
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Image {
        pub height: u32,
        pub url: String,
        pub width: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ExternalUrls {
        pub spotify: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Followers {
        pub href: Option<String>,
        pub total: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ExternalIds {
        pub isrc: String,
    }

    

}
