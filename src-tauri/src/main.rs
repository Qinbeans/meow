use std::sync::Mutex;

mod youtube;
use discord::discord::{
  SimpleDiscord
};
mod discord;

struct DiscordStorage{
  discord: Mutex<SimpleDiscord>
}

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
client.discord.
fn main() {
  let client = discord::new();
  if client.is_none(){
    println!("Failed to initialize meow token");
    return;
  }
  let meow_client = client.unwrap();
  tauri::Builder::default()
    //call discord api request
    .manage(DiscordStorage { discord: meow_client } )
    .invoke_handler(tauri::generate_handler![youtube])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(async)]
async fn youtube(url: String, client: tauri::State<DiscordStorage>) -> Result<String,String> {
  let client_raw = client.discord.lock().unwrap();
  let token_raw = client_raw.token;
  if token_raw.is_none(){
    println!("Failed to get meow token");
    return Err("No token found".to_string());
  }
  let token = token_raw.unwrap();
  let results = youtube::youtube::query_youtube(&url, token).await;
  
  if let Err(e) = results {
    println!("Search Error: {}", e);
    return Err(format!("Search Error: {}", e));
  }
  let results = results.ok().unwrap();
  //turn SimpleYoutube into a json string
  let json_raw = serde_json::to_string(&results);
  if let Err(e) = json_raw {
    println!("Result Error: {}", e);
    return Err(format!("Result Error: {}", e));
  }
  let json = json_raw.unwrap();
  Ok(json)
}