use crate::yt_audio::error::BlobResult;

use super::{config, get_output_path};

/// Returns a ConfigYtVideo object with all the necessary data
/// to start downloading a youtube video
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &str) -> BlobResult<config::DownloadConfig> {
    let output_path = get_output_path()?.trim().to_string();

    Ok(config::DownloadConfig::new_video(url, output_path.into()))
}
