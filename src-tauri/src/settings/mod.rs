use std::fs;
pub mod client;

pub fn read_settings(path: &str) -> Result<client::Client,String>
{
   let res_read = fs::read_to_string(path);
   let mut json_string: String;
   if let Err(_) = res_read {
      //create file
      println!("Missing settings, creating...");
      json_string = "{\"youtube_api_key\":\"\",\"id\":\"\",\"token\":\"\"}".to_string();
      let res_write = fs::write(path,json_string);
      if let Err(ref e) = res_write {
         return Err(format!("{}",e));
      }
   }
   json_string = res_read.unwrap();
   let res_json = serde_json::from_str(&json_string);
   if let Err(e) = res_json {
      return Err(format!("{}",e));
   }
   let client = res_json.unwrap();
   Ok(client)
}