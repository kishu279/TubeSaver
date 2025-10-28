use clap::{Arg, Command};
extern crate google_drive3 as drive3;
use drive3::{DriveHub, api::File as DriveFile, hyper_rustls, hyper_util, yup_oauth2};
use regex::Regex;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use walkdir::WalkDir;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

#[tokio::main]
async fn main() {
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
                // .required(true)
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
        .arg(
            Arg::new("upload")
                .short('g')
                .long("g-upload")
                .num_args(0)
                .help("upload folder on google drive"),
        )
        // .arg(Arg::new("download").short('d').long("gdrive-download"))
        .get_matches();

    let mode = matches
        .get_one::<String>("mode")
        .map(|s| s.as_str())
        .unwrap_or("video");
    let mute = matches.get_flag("mute");
    let url = matches.get_one::<String>("url");
    let upload = matches.get_flag("upload");
    let path = matches.get_one::<String>("path").map(|p| p.as_str());
    // creating variable for retreving video id further
    let mut _video_id = String::new();

    if upload == true
        && mute == false
        && let Some(path) = path
    {
        println!("Uploading to Google Drive");
        // upload_to_gdrive(url.to_string()).expect("failed to upload the data");
        upload_to_gdrive(path.to_string())
            .await
            .expect("failed to upload the file");
    } else if let Some(url) = url {
        match extract_video_id(url) {
            Some(video_id) => println!("Extracted video ID: {video_id}"),
            None => println!("Could not extract video ID from the URL"),
        }
        println!("Mode: {mode}, URL: {url}, Mute: {mute}");
        //calling the download from url function
        download_from_url(url.to_string()).expect("failed to download the data");
    } else {
        println!("Add an URL or a path to download / upload");
        return;
    }
}

//function to extract the video id
fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(?:https?://(?:www\.)?youtube\.com/(?:[^/]+/.*?v=|(?:v|e(?:mbed)?|feeds/api/videos|user/[^/]+/videos|watch\?v=))|youtu\.be/)(?P<id>[a-zA-Z0-9_-]{11})").unwrap();
    re.captures(url)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

//this brings on asyncchronous operation in rust

// #[tokio::main]
// //creating a asyncchronous function to download the video
// pub async fn download_video(videourl: String) -> Result<(), Box<dyn std::error::Error>> {
//     // setting up the binaries for yt-dlp and ffmpeg
//     let executables_dir = PathBuf::from("libs");
//     let output_dir = PathBuf::from("downloads");

//     //downloading the binaries
//     let fetchervar = Youtube::with_new_binaries(&executables_dir, &output_dir).await?;

//     //updating the binaries
//     fetchervar.update_downloader().await?;

//     //last try
//     let (yt_dlp_name, ffmpeg_name) = ("yt-dlp", "ffmpeg");

//     let youtube_path = executables_dir.join(yt_dlp_name);
//     let ffmpeg_path = executables_dir.join(ffmpeg_name);

//     println!("Using yt-dlp at: {}", youtube_path.display());
//     println!("Using ffmpeg at: {}", ffmpeg_path.display());
//     //main data fetching logic
//     // let libraries_dir = PathBuf::from("libs");
//     // let output_dir = PathBuf::from("downloads");

//     // let youtube = libraries_dir.join("yt-dlp");
//     // let ffmpeg = libraries_dir.join("ffmpeg");

//     let libraries = Libraries::new(youtube_path, ffmpeg_path);

//     let fetcher = Youtube::new(libraries, output_dir)?;

//     //using the string
//     let url = String::from(videourl);
//     println!("the video url is : {}", url);

//     //adding error too in this field ( have to add )
//     let video_path = fetcher.download_video_from_url(url, "video.mp4").await?; //pass the message to the user
//     println!("video successfully downloaded");
//     println!("path to video {}", video_path.to_string_lossy());

//     Ok(())
// }

//using

// fn download_from_url(video_url: String) -> Result<(), Box<dyn std::error::Error>> {
//     //creating the output directory

//     let output_template = "downloads/%(title)s.%(ext)s";
//     std::fs::create_dir_all(&output_template)?;

//     // let outputdir = "downloads";
//     println!("Starting yt-dlp to download: {}", video_url);

//     //check if yt-dlp  is present
//     if !check_ytdlp_installed() {
//         return Err("yt-dlp not instaled".into());
//     }

//     // Create and configure the command
//     let mut command = ProcessCommand::new("yt-dlp");
//     command
//         .arg("-o")
//         .arg(output_template)
//         .arg("--no-playlist")
//         .arg("mp4")
//         .arg(video_url);

//     // as child process to execute the system commands
//     let output = command.output()?;

//     //check for success
//     if output.status.success() {
//         println!("Download successfull")
//     } else {
//         eprintln!("Download failed")
//     }

//     Ok(())
// }

// fn check_ytdlp_installed() -> bool {
//     ProcessCommand::new("yt-dlp")
//         .arg("--version")
//         .output()
//         .map(|output| output.status.success())
//         .unwrap_or(false)
// }

fn download_from_url(video_url: String) -> Result<(), Box<dyn Error>> {
    // Create output directory
    let output_dir = "downloads";
    std::fs::create_dir_all(output_dir)?;

    // Define output file template
    let output_template = format!("{}/%(title)s.%(ext)s", output_dir);
    println!("Starting yt-dlp to download: {}", video_url);

    // Check if yt-dlp is installed
    if !check_ytdlp_installed() {
        return Err("yt-dlp not installed".into());
    }

    // Build command
    let mut command = ProcessCommand::new("yt-dlp");
    command
        .arg("-o")
        .arg(&output_template)
        .arg("--no-playlist")
        .arg("-f")
        .arg("mp4")
        .arg(&video_url);

    // Execute command
    let output = command.output()?;

    // Log results
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        println!("✅ Download successful");
    } else {
        eprintln!("❌ Download failed");
    }

    Ok(())
}

fn check_ytdlp_installed() -> bool {
    ProcessCommand::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

async fn upload_to_gdrive(file_path: String) -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let oauth_path = format!("{}/.tubesaver/google_oauth.json", home);
    let tokens_path = format!("{}/.tubesaver/tokens.json", home);

    let secret = yup_oauth2::read_application_secret(&oauth_path).await?;
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::Interactive)
        .persist_tokens_to_disk(&tokens_path)
        .build()
        .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    let hub = DriveHub::new(client, auth);

    let path = Path::new(&file_path);
    if path.is_dir() {
        println!("📁 Uploading directory: {}", path.display());
        for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                upload_file(&hub, &entry.path().to_path_buf())
                    .await
                    .expect("failed to upload file");
            }
        }
    } else if path.is_file() {
        println!("📄 Uploading file: {}", path.display());
        upload_file(&hub, &path.to_path_buf()).await?;
    } else {
        return Err(format!("Path not found or invalid: {}", file_path).into());
    }

    Ok(())
}

async fn upload_file<T>(hub: &DriveHub<T>, path: &PathBuf) -> Result<(), Box<dyn Error>>
where
    T: 'static + Send + Sync + Clone + hyper_util::client::legacy::connect::Connect,
{
    let file_name = path.file_name().unwrap().to_string_lossy().to_string();
    let file = std::fs::File::open(path)?;

    let metadata = DriveFile {
        name: Some(file_name),
        ..Default::default()
    };
    let (_response, _res) = hub
        .files()
        .create(metadata)
        .upload(file, "application/octet-stream".parse()?)
        .await?;

    println!("✅ Uploaded: {}", path.display());
    Ok(())
}
