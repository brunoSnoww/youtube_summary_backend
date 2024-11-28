use which::which;

use super::dispatcher::dispatch;

pub fn get_yt_audio(url: &str) {
    if which("yt-dlp").is_ok() {
        if let Err(err) = dispatch(url.into()) {
            err.report();
        }
    } else {
        // ytdlp is not installed!
        panic!("yt-dlp not installed");
    }
}
