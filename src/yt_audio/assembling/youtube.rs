pub mod config;
pub mod yt_video;

use serde::{Deserialize, Serialize};
use std::{env, fmt};

// Functions used both in yt_video.rs and yt_playlist.rs
/// Asks the user whether they want to download video files or audio-only
fn get_media_selection() -> Result<MediaSelection, std::io::Error> {
    Ok(MediaSelection::AudioOnly)
}

fn get_output_path() -> BlobResult<String> {
    Ok(env::current_dir()?.as_path().display().to_string())
}

use crate::yt_audio::error::BlobResult;

#[derive(Clone, Debug)]
pub(crate) enum MediaSelection {
    FullVideo,
    VideoOnly,
    AudioOnly,
}

/// All the information about a particular video format
#[derive(Deserialize, Serialize, Debug, PartialOrd, PartialEq)]
struct VideoFormat {
    format_id: String,
    // File extension
    ext: String,
    // Fps count, is null for audio-only formats
    fps: Option<f64>,
    // How many audio channels are available, is null for video-only formats. Unavailable on weird sb* formats
    audio_channels: Option<u64>,
    // Video resolution, is "audio only" for audio-only formats
    resolution: String,
    // Measured in MB. Unavailable on sb* formats
    filesize: Option<u64>,
    // Video codec, can be "none"
    vcodec: String,
    // Audio codec, can be "none" or straight up not exist (like in mp4 audio-only formats)
    acodec: Option<String>,
    // Codec container
    container: Option<String>,
    // Total average bitrate
    tbr: Option<f64>,
    // When filesize is null, this may be available
    filesize_approx: Option<u64>,
}

// A list of all the formats available for a single video
#[derive(Deserialize, Serialize, Debug)]
struct VideoSpecs {
    formats: Vec<VideoFormat>,
}

#[derive(Debug, Clone)]
/// What quality and format the user wants a specific video to be downloaded in
pub(crate) enum VideoQualityAndFormatPreferences {
    // Code of the selected format
    UniqueFormat(String),
    // Recode the downloaded file to this format (post-processor)
    ConvertTo(String),
    BestQuality,
    SmallestSize,
}

impl fmt::Display for VideoFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result;

        if let Some(tbr) = self.tbr {
            // Skip picture formats
            // Add container
            result = format!("{:<6} ", self.ext);

            if self.resolution != "audio only" {
                result = format!("{}| {:<13} ", result, self.resolution);
            }

            // This isn't a picture format so unwrap() is safe
            let filesize = self.filesize.unwrap_or(0);

            // filesize is converted from bytes to MB
            let filesize_section = format!("| filesize: {:<.2}MB", filesize as f32 * 0.000001);
            result = format!("{}{:<24}", result, filesize_section);

            // If available, add audio channels
            if let Some(ch) = self.audio_channels {
                result = format!("{}| {} audio ch ", result, ch);
            }

            result = format!("{}| tbr: {:<8.2} ", result, tbr);

            if self.vcodec != "none" {
                result = format!("{}| vcodec: {:<13} ", result, self.vcodec);
            }

            if let Some(acodec) = &self.acodec {
                if acodec != "none" {
                    result = format!("{}| acodec: {:<13} ", result, acodec);
                }
            }

            #[cfg(debug_assertions)]
            return {
                result = format!("[[DEBUG code: {:<3}]] {} ", self.format_id, result);
                write!(f, "{}", result)
            };

            #[cfg(not(debug_assertions))]
            write!(f, "{}", result)
        } else {
            write!(f, "I shouldn't show up because I am a picture format")
        }
    }
}

impl VideoSpecs {
    fn formats(&self) -> &Vec<VideoFormat> {
        &self.formats
    }
}
