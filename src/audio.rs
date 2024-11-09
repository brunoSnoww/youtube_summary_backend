use regex::Regex;
use std::io;
use std::process::Command;
/// Resamples an audio file to 16,000 Hz and converts it to mono using SoX.
///
/// # Arguments
///
/// * `input_file` - The path to the input audio file.
/// * `output_file` - The path where the resampled audio will be saved.
///
/// # Returns
///
/// * `Ok(())` if the command executes successfully.
/// * `Err(io::Error)` if there is an error executing the command.
pub fn resample_audio(input_file: &str, output_file: &str) -> io::Result<()> {
    let status = Command::new("sox")
        .args([input_file, "-r", "16000", "-c", "1", output_file])
        .status()?;

    if status.success() {
        println!("Audio resampled successfully to {}", output_file);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("SoX command failed with status: {}", status),
        ))
    }
}

pub fn convert_mp4_to_wav(input_path: &str, output_path: &str) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-vn",
            "-acodec",
            "pcm_s16le",
            "-ar",
            "44100",
            "-ac",
            "2",
            output_path,
        ])
        .status()?;

    if status.success() {
        println!("Conversion successful!");
    } else {
        eprintln!("Conversion failed.");
    }

    Ok(())
}

pub fn transcribe_audio(audio_path: &str, model_path: &str) -> io::Result<String> {
    let output =
        Command::new("/Users/brunoneves/Desktop/personalProjects/yt_summarizer/whisper.cpp/main")
            .args(["-m", model_path, "-f", audio_path])
            .output()?;

    if output.status.success() {
        let transcription = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(transcription)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("{error_message}");
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Transcription failed: {}", error_message),
        ))
    }
}

/// Processes the transcript by optionally removing timestamps.
///
/// # Arguments
///
/// * `transcript` - The transcript text containing timestamps and spoken text.
/// * `remove_timestamps` - A boolean flag indicating whether to remove timestamps.
///
/// # Returns
///
/// * A `String` containing the processed transcript.
pub fn process_transcript(transcript: &str, remove_timestamps: bool) -> String {
    if remove_timestamps {
        // Remove timestamps and extract the spoken text
        extract_text(transcript)
    } else {
        // Return the transcript as is
        transcript.to_string()
    }
}

/// Extracts the spoken text from the transcript by removing timestamps.
///
/// # Arguments
///
/// * `transcript` - The transcript text containing timestamps and spoken text.
///
/// # Returns
///
/// * A `String` containing only the spoken text.
fn extract_text(transcript: &str) -> String {
    // Regular expression to match lines with timestamps
    let re =
        Regex::new(r"^\[\d{2}:\d{2}:\d{2}\.\d{3} --> \d{2}:\d{2}:\d{2}\.\d{3}\]\s+(.*)$").unwrap();
    let mut extracted_text = String::new();

    for line in transcript.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(text_match) = caps.get(1) {
                extracted_text.push_str(text_match.as_str());
                extracted_text.push(' ');
            }
        } else {
            // If the line doesn't match, include it as is (handles any additional text)
            extracted_text.push_str(line);
            extracted_text.push(' ');
        }
    }

    extracted_text.trim().to_string()
}
