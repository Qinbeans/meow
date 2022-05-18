mod settings;

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

const SETTINGS:&str = "settings.json";

fn main() {
  //read file settings.json
  let client = settings::read_settings(SETTINGS);
  if let Err(e) = client {
    println!("{}",e);
    return;
  }
  let client = client.unwrap();
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
