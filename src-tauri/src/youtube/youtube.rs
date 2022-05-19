use serde::{Deserialize, Serialize};
use yt_api::{
    search::{
        SearchList,
        ItemType,
    },
    ApiKey
};

const MAX_QUERY: u8 = 25;

#[derive(Serialize, Deserialize)]
pub struct SimpleYoutube {
    name: String,
    thumbnail: String,
    id: String,
}

impl SimpleYoutube {
    pub fn new(name: String, thumbnail: String, id: String) -> SimpleYoutube {
        SimpleYoutube {
            name,
            thumbnail,
            id,
        }
    }
}

pub async fn query_youtube(query: &str, api: &str) -> Result<Vec<SimpleYoutube>,String>{
    //query endpoint from bot -> Return JSON from endpoint
    let results = SearchList::new(ApiKey::new(api))
        .item_type(ItemType::Video)
        .max_results(MAX_QUERY)
        .q(query)
        .await;
    if let Err(e) = results {
        return Err(format!("{}", e));
    }
    let results = results.ok().unwrap();
    let items = results.items;
    let mut simplet_yt:Vec<SimpleYoutube> = Vec::new();
    for item in items {
        let name = item.snippet.title.unwrap_or("".to_string());
        let thumbnail:String = {
            if (&item.snippet.thumbnails).is_none(){
                "".to_string();
            }
            if (&item.snippet.thumbnails.as_ref().unwrap()).default.is_none(){
                "".to_string();
            }
            item.snippet.thumbnails.unwrap().default.unwrap().url.clone()
        };
        let id = item.id.video_id.unwrap_or("".to_string());
        simplet_yt.push(SimpleYoutube::new(name, thumbnail, id));
    }
    Ok(simplet_yt)
}