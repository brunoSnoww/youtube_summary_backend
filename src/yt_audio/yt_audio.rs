use which::which;

use super::{dispatcher::dispatch, parser};

pub fn get_yt_audio(url: &str) {
    let output_path =
        "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/youtube_summary_backend"
            .to_string();
    // tested with yt-dlp 2023.07.06
    if which("yt-dlp").is_ok() {
        // check whether yt-dlp's version is compatible with this version of blob-dl

        if let Err(err) = dispatch(url.into(), output_path) {
            // Tell the user about the error
            err.report();
        }
    } else {
        // ytdlp is not installed!
    }
}
