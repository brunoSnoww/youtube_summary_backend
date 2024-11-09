use super::{
    analyzer,
    assembling::{self, youtube::config::DownloadConfig},
    error::BlobResult,
    parser::{self, Verbosity},
    run,
};

/// Calls the builder function according to what the url refers to (video/playlist), then it runs the ytdl-command and handles errors
pub fn dispatch(url: String, output_path: String) -> BlobResult<()> {
    // Parse what the url refers to
    //
    let download_config = DownloadConfig {
        url: url.clone(),
        output_path,
        include_indexes: false,
        chosen_format: assembling::youtube::VideoQualityAndFormatPreferences::SmallestSize,
        media_selected: assembling::youtube::MediaSelection::AudioOnly,
        download_target: analyzer::DownloadOption::YtVideo(0),
    };
    let download_option = analyzer::analyze_url(&url);

    // Generate a command according to the user's preferences
    let mut command_and_config = assembling::generate_command(&url, &download_option?)?;
    // Run the command
    run::run_and_observe(
        &mut command_and_config.0,
        &download_config,
        &Verbosity::Quiet,
    );

    Ok(())
}
