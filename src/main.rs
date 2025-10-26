extern crate dotenv;
use clap::{Arg, Command};
use regex::Regex;
use dotenv::dotenv;
use std::env;

fn main() {
    let matches = Command::new("TubeSaver")
        .version("1.0")
        .about("Download YouTube videos easily")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .default_value("video")
                .help("mode: video or audio"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .required(true)
                .help("YouTube video URL"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Output file path"),
        )
        .arg(
            Arg::new("quality")
                .short('q')
                .long("quality")
                .default_value("1080p"),
        )
        .arg(
            Arg::new("mute")
                .short('j')
                .long("mute")
                .num_args(0)
                .help("mute audio"),
        )
        .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();
    let url = matches.get_one::<String>("url").unwrap();
    let mute = matches.get_flag("mute");
    // creating variable for retreving video id further
    let mut _video_id  = String::new();
    match extract_video_id(url) {
        Some(video_id) => println!("Extracted video ID: {video_id}"),
        None => println!("Could not extract video ID from the URL"),
    }

    println!("Mode: {mode}, URL: {url}, Mute: {mute}");
   
   //calling the download video function in rust
 download_video();

}

fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(?:https?://(?:www\.)?youtube\.com/(?:[^/]+/.*?v=|(?:v|e(?:mbed)?|feeds/api/videos|user/[^/]+/videos|watch\?v=))|youtu\.be/)(?P<id>[a-zA-Z0-9_-]{11})").unwrap();
    re.captures(url)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

fn download_video() {
 //check for the env files like if the data is available in the system
 dotenv().ok();

 //check for the youtube credential
 let client_id = env::var("YOUTUBE_API_KEY").expect("client id not found");

 //if client id  found 
 if client_id != ""  {

}
}