use which::which;

use super::{dispatcher::dispatch, parser};

pub fn get_yt_audio(url: &str) {
    let config = parser::parse_config();
    let output_path = "/Users/brunoneves/Desktop/personalProjects/blob-dl".to_string();
    // tested with yt-dlp 2023.07.06
    if which("yt-dlp").is_ok() {
        // check whether yt-dlp's version is compatible with this version of blob-dl

        match config {
            Ok(config) => {
                // Ask for more input > Generate a command > Execute yt-dlp
                if let Err(err) = dispatch(&config, url.into(), output_path) {
                    // Tell the user about the error
                    err.report();
                }
            }
            Err(err) => {
                err.report();
            }
        }
    } else {
        // ytdlp is not installed!
    }
}
