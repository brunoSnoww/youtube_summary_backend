use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use which::which;

use crate::yt_audio::assembling::youtube::*;
use crate::yt_audio::error::BlobResult;

/// Returns a ConfigYtVideo object with all the necessary data
/// to start downloading a youtube video
///
/// Takes in the command line arguments list
pub(crate) fn assemble_data(url: &str, playlist_id: usize) -> BlobResult<config::DownloadConfig> {
    let term = Term::buffered_stderr();

    // Whether the user wants to download video files or audio-only
    let media_selected = get_media_selection(&term)?;

    let chosen_format = format::get_format(&term, url, &media_selected, playlist_id)?;

    // .trim() trims trailing whitespace at the end of the user-specified path (useful is the user is clumsy)
    let output_path = get_output_path(&term)?.trim().to_string();

    Ok(config::DownloadConfig::new_video(
        url,
        chosen_format,
        output_path,
        media_selected,
    ))
}

mod format {
    use crate::yt_audio::error::{
        BEST_QUALITY_PROMPT_SINGLE_VIDEO, CONVERT_FORMAT_PROMPT_VIDEO_SINGLE_VIDEO,
        FFMPEG_UNAVAILABLE_WARNING, SMALLEST_QUALITY_PROMPT_SINGLE_VIDEO,
        YT_FORMAT_PROMPT_SINGLE_VIDEO,
    };

    use super::*;

    /// Asks the user to choose a download format and quality between the ones
    /// available for the current video.
    ///
    /// The options are filtered between video, audio-only and video-only
    pub(super) fn get_format(
        term: &Term,
        url: &str,
        media_selected: &MediaSelection,
        playlist_id: usize,
    ) -> BlobResult<VideoQualityAndFormatPreferences> {
        Ok(VideoQualityAndFormatPreferences::SmallestSize)
    }

    /// Presents the user with the formats youtube provides directly for download, without the need for ffmpeg
    fn get_format_from_yt(
        term: &Term,
        url: &str,
        media_selected: &MediaSelection,
        playlist_id: usize,
    ) -> BlobResult<VideoQualityAndFormatPreferences> {
        // Serialize all available formats from the youtube API (through yt-dlp -F)
        let serialized_formats = {
            // Get a JSON dump of all the available formats for the current url
            let ytdl_formats = get_ytdlp_formats(url)?;

            // Serialize the JSON which contains the format information for the current video
            serialize_formats(
                std::str::from_utf8(&ytdl_formats.stdout[..])?
                    // If `url` refers to a playlist the JSON has multiple roots, only parse one
                    .lines()
                    // If the requested video isn't the first in a playlist, only parse its information
                    .nth(playlist_id - 1)
                    // Unwrap is safe because playlist_id is non-0 only when there are multiple lines in the json
                    .unwrap(),
            )?
        };

        // Ids which the user can pick according to the current media selection
        let mut correct_ids = vec![];
        // Every format which conforms to media_selected will be pushed here
        let mut format_options = vec![];

        // Choose which formats to show to the user
        for format in serialized_formats.formats() {
            // If format and media_selected are compatible
            if check_format(format, media_selected) {
                // Add to the list of available formats the current one formatted in a nice way
                format_options.push(format.to_string());
                // Update the list of ids which match what the user wants
                correct_ids.push(format.format_id.clone());
            }
        }

        // Set up a prompt for the user
        let user_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which format do you want to apply to the video?")
            .default(0)
            .items(&format_options)
            .interact_on(term)?;

        // Return the format corresponding to what the user selected, the choices are limited so there shouldn't be out-of-bounds problems
        Ok(VideoQualityAndFormatPreferences::UniqueFormat(
            correct_ids[user_selection].clone(),
        ))
    }
}
