use clap::{Arg, Command};
use regex::Regex;
use std::path::PathBuf;
use yt_dlp::Youtube;
use yt_dlp::fetcher::deps::Libraries;
use anyhow::Result;

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
    let mut _video_id = String::new();
    match extract_video_id(url) {
        Some(video_id) => println!("Extracted video ID: {video_id}"),
        None => println!("Could not extract video ID from the URL"),
    }

    println!("Mode: {mode}, URL: {url}, Mute: {mute}");

    //calling the download video function in rust
    let _ = download_video(url.to_string());

    //calling the download from url function
    download_from_url(url.to_string()).expect("failed to download the data");

}

fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(?:https?://(?:www\.)?youtube\.com/(?:[^/]+/.*?v=|(?:v|e(?:mbed)?|feeds/api/videos|user/[^/]+/videos|watch\?v=))|youtu\.be/)(?P<id>[a-zA-Z0-9_-]{11})").unwrap();
    re.captures(url)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

//this brings on asyncchronous operation in rust
#[tokio::main]
//creating a asyncchronous function to download the video
pub async fn download_video(videourl: String) -> Result<(), Box<dyn std::error::Error>> {
    // setting up the binaries for yt-dlp and ffmpeg
    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("downloads");

    //downloading the binaries
    let fetchervar = Youtube::with_new_binaries(&executables_dir, &output_dir).await?;

    //updating the binaries
    fetchervar.update_downloader().await?;

    //last try 
   let (yt_dlp_name, ffmpeg_name) = ("yt-dlp", "ffmpeg");
    
    let youtube_path = executables_dir.join(yt_dlp_name);
    let ffmpeg_path = executables_dir.join(ffmpeg_name);
    
    println!("Using yt-dlp at: {}", youtube_path.display());
    println!("Using ffmpeg at: {}", ffmpeg_path.display());
    //main data fetching logic
    // let libraries_dir = PathBuf::from("libs");
    // let output_dir = PathBuf::from("downloads");

    // let youtube = libraries_dir.join("yt-dlp");
    // let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube_path, ffmpeg_path);

    let fetcher = Youtube::new(libraries, output_dir)?;

    //using the string
    let url = String::from(videourl);
    println!("the video url is : {}", url);

    //adding error too in this field ( have to add )
    let video_path = fetcher.download_video_from_url(url, "video.mp4").await?; //pass the message to the user
    println!("video successfully downloaded");
    println!("path to video {}", video_path.to_string_lossy());

    Ok(())
}


//using 
fn download_from_url (video_url : String)-> Result<()>{

  //creating the output directory
    let output_template:&str ="downloads";
    std::fs::create_dir_all(&output_template)?;

    println!("Starting yt-dlp to download: {}", video_url);

    // Create and configure the command
    let mut command = Command::new("yt-dlp");
    command
        .arg("--output")
        .arg(output_template)
        .arg("--format")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]")
        .arg(video_url);

    // Run the command and wait for it to complete
    let output = command.output()?;

    // Check if the command was successful
    if output.status.success() {
        println!("yt-dlp command ran successfully!");
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("yt-dlp command failed with an error!");
        eprintln!("status: {}", output.status);
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    }

//once completed delete the download folder
    Ok(())
}

fn check_ytdlp_installed() -> bool {
    ProcessCommand::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}