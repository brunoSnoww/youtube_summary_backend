use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use super::error::YtdlpError;

pub fn run_command(command: &mut Command) -> Option<Vec<YtdlpError>> {
    // Run the command and capture its output
    let mut youtube_dl = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start yt-dlp process"); // TODO Should take away this expect in a future release

    let stdout = BufReader::new(youtube_dl.stdout.take().unwrap());
    let stderr = BufReader::new(youtube_dl.stderr.take().unwrap());

    // All the errors produced by yt-dlp
    let mut errors: Vec<YtdlpError> = vec![];

    // This has to be run or the command does nothing
    for line in stdout.lines().chain(stderr.lines()) {
        let line = line.unwrap();

        // Keep track of errors without displaying anything
        if line.contains("ERROR:") {
            errors.push(YtdlpError::from_error_output(&line));
        }
    }

    if errors.is_empty() {
        None
    } else {
        Some(errors)
    }
}
