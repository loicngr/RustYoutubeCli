extern crate dotenv;
extern crate reqwest;
extern crate serde_json;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug)]
struct Cli {
    msg: String,
    params: Vec<String>,
    youtube_api_key: String
}

impl Cli {
    fn new(msg: String, params: Vec<String>, youtube_api_key: String) -> Self { 
        Self { msg, params, youtube_api_key }
    }
    async fn get_playlist_uploads(&self, channel_id: String) -> Result<String, reqwest::Error> {
        let base_url: String = "https://www.googleapis.com/youtube/v3/channels/".to_owned();
        let client = reqwest::Client::new();
        let resp = client
            .get(format!("{}?part=snippet,contentDetails,statistics&id={}&key={}", &base_url, &channel_id, &self.youtube_api_key).as_str())
            .send()
            .await?;
        let text = resp.text().await?;
        let video: serde_json::Value = serde_json::from_str(&text).unwrap();
        let channel_playlists = video.get("items").unwrap()[0].get("contentDetails").unwrap().get("relatedPlaylists").unwrap();
        let channel_uploads = channel_playlists.get("uploads").unwrap();
        Ok(channel_uploads.to_string().replace("\"", ""))
    }
    async fn get_video_by_id(&self, playlist_id: String) -> Result<serde_json::Value, reqwest::Error> {
        let base_url: String = "https://www.googleapis.com/youtube/v3/playlistItems".to_owned();
        let url: String = format!("{}?part=snippet&playlistId={}&maxResults=1&key={}", &base_url, &playlist_id.as_str(), &self.youtube_api_key);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .send()
            .await?;
        let text = resp.text().await?;
        let video: serde_json::Value = serde_json::from_str(&text).unwrap();
        Ok(video)
    }
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let youtube_api_key = dotenv::var("YOUTUBE_API_KEY").unwrap();

    let args: Vec<String> = env::args().collect();
    let cli = Cli::new(args[0].trim().to_owned(), args, youtube_api_key);

    for param in &cli.params {
        let full_param: Vec<&str> = param.split("=").collect();

        match full_param[0] {
            "lastvideo-user" => {
                if full_param.len() == 2usize {
                    let channel_id: String = full_param[1].parse().unwrap();
                    let channel = cli.get_playlist_uploads(channel_id).await?;
                    let video = cli.get_video_by_id(channel).await?;
                    println!("{}", video);
                } else {
                    println!("Please, give channel id like this : lastvideo-user=UCdBpdsdmd55444dKy42ExpEw");
                }
            }
            "help" => {
                println!("Please, see the readme file for help.");
            }
            _ => {}
        }
    }
    Ok(())
}