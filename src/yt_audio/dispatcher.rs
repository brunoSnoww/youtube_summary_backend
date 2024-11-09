use super::{
    analyzer,
    assembling::{self, youtube::config::DownloadConfig},
    error::BlobResult,
    parser, run,
};

/// Calls the builder function according to what the url refers to (video/playlist), then it runs the ytdl-command and handles errors
pub fn dispatch(config: &parser::CliConfig, url: String, output_path: String) -> BlobResult<()> {
    // Parse what the url refers to
    //
    let download_config = DownloadConfig {
        url,
        output_path,
        include_indexes: false,
        chosen_format: assembling::youtube::VideoQualityAndFormatPreferences::SmallestSize,
        media_selected: assembling::youtube::MediaSelection::AudioOnly,
        download_target: analyzer::DownloadOption::YtVideo(0),
    };
    let download_option = analyzer::analyze_url(config.url());

    // Generate a command according to the user's preferences
    let mut command_and_config = assembling::generate_command(config.url(), &download_option?)?;

    if config.show_command() {
        println!("Command generated by blob-dl: {:?}", command_and_config.0);
    }
    // Run the command
    run::run_and_observe(
        &mut command_and_config.0,
        &download_config,
        config.verbosity(),
    );

    Ok(())
}