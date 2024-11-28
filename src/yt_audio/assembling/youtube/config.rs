use crate::yt_audio::analyzer;
use std::{path::PathBuf, process};

/// Contains all the information needed to download a YouTube video or playlist.
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    url: String,
    output_path: PathBuf,
    pub download_target: analyzer::DownloadOption,
}

impl DownloadConfig {
    pub(crate) fn new_video(url: &str, output_path: PathBuf) -> DownloadConfig {
        DownloadConfig {
            url: url.to_string(),
            output_path,
            download_target: analyzer::DownloadOption::YtVideo(0),
        }
    }

    /// Builds a command according to the current configuration, which is also returned.
    pub(crate) fn build_command(&self) -> (process::Command, DownloadConfig) {
        let command = match self.download_target {
            analyzer::DownloadOption::YtVideo(_) => self.build_yt_video_command(),
            analyzer::DownloadOption::YtPlaylist => self.build_yt_playlist_command(),
        };
        (command, self.clone())
    }

    fn build_yt_playlist_command(&self) -> process::Command {
        let mut command = self.initialize_command();
        command.arg("-i").arg("--yes-playlist");
        self.choose_output_path(&mut command);
        self.choose_format(&mut command);
        command.arg(&self.url);
        command
    }

    fn build_yt_video_command(&self) -> process::Command {
        let mut command = self.initialize_command();
        self.choose_output_path(&mut command);
        self.choose_format(&mut command);
        command.arg("--no-playlist").arg(&self.url);
        command
    }

    pub fn build_command_for_video(&self, video_id: &str) -> process::Command {
        let mut command = self.initialize_command();
        self.choose_output_path(&mut command);
        self.choose_format(&mut command);
        command.arg("--no-playlist").arg(video_id);
        command
    }

    fn choose_output_path(&self, command: &mut process::Command) {
        let path_and_scheme = format!("{}/audio.%(ext)s", self.output_path.display());
        command.arg("-o").arg(path_and_scheme);
    }

    fn choose_format(&self, command: &mut process::Command) {
        command.arg("-f").arg("worstaudio");
    }

    fn initialize_command(&self) -> process::Command {
        process::Command::new("yt-dlp")
    }
}
