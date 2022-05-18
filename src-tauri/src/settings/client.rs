use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Client{
   youtube_api_key: String,
   id: String,
   token: String,
}