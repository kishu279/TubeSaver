use std::path;

use clap::{Arg, Command};

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
                .long("quit")
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

    println!("Mode: {mode}, URL: {url}, Mute: {mute}");
}
