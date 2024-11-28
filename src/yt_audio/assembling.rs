use youtube::yt_video::assemble_data;

use super::error::BlobResult;

pub mod youtube;

pub(crate) fn generate_command(
    url: &str,
) -> BlobResult<(std::process::Command, youtube::config::DownloadConfig)> {
    let assemble = assemble_data(url)?;

    let (command, local_config) = assemble.build_command();
    Ok((command, local_config))
}
